//! `chrono-rkyv-migrate` command-line adapter.

use std::ffi::OsString;
use std::fs::{self, File, OpenOptions};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use bpaf::{construct, long, OptionParser, Parser};
use chrono_rkyv_migrate::{
  ArchiveFormat, Endianness, MigrationError, PointerWidth, SourceHint,
};

/// Parsed `migrate` command-line surface.
#[derive(Debug)]
struct Options {
  /// Direct chrono root type.
  archive_type:   String,
  /// Source archive generation.
  source_version: String,
  /// Input archive path.
  input:          PathBuf,
  /// Separate destination path.
  output:         Option<PathBuf>,
  /// Atomically replace the input.
  in_place:       bool,
  /// Permit replacement of an existing separate output.
  replace:        bool,
}

/// Command-line execution failure.
#[derive(Debug, thiserror::Error)]
enum CliError {
  /// Invalid option combination.
  #[error("{0}")]
  InvalidOptions(String),
  /// Input file could not be read.
  #[error("failed to read `{path}`: {source}")]
  Read {
    /// Input path.
    path:   PathBuf,
    /// Filesystem failure.
    source: io::Error,
  },
  /// Archive migration failed.
  #[error(transparent)]
  Migration(#[from] MigrationError),
  /// Output could not be opened or written.
  #[error("failed to write `{path}`: {source}")]
  Write {
    /// Destination path.
    path:   PathBuf,
    /// Filesystem failure.
    source: io::Error,
  },
  /// In-place replacement could not be completed atomically.
  #[error("failed to replace `{path}` atomically: {source}")]
  Replace {
    /// Original input path.
    path:   PathBuf,
    /// Filesystem failure.
    source: io::Error,
  },
}

/// Parsed destination selection.
enum Destination {
  /// Write a distinct output path.
  Output(PathBuf),
  /// Replace the input atomically.
  InPlace,
}

fn main() -> ExitCode {
  match execute(options().run()) {
    Ok(()) => ExitCode::SUCCESS,
    Err(error) => {
      eprintln!("chrono-rkyv-migrate: {error}");
      ExitCode::FAILURE
    }
  }
}

/// Builds the maintained `bpaf` command parser.
fn options() -> OptionParser<Options> {
  let archive_type = long("type")
    .help("Direct chrono root type")
    .argument::<String>("SUPPORTED_CHRONO_TYPE");
  let source_version = long("source-version")
    .help("Source archive generation: auto, 0.7, or 0.8")
    .argument::<String>("VERSION");
  let input = long("input")
    .help("Input archive path")
    .argument::<PathBuf>("PATH");
  let output = long("output")
    .help("Separate destination path")
    .argument::<PathBuf>("PATH")
    .optional();
  let in_place = long("in-place")
    .help("Atomically replace the input after successful migration")
    .switch();
  let replace = long("replace")
    .help("Permit replacement of an existing separate output")
    .switch();

  construct!(Options {
    archive_type,
    source_version,
    input,
    output,
    in_place,
    replace
  })
  .to_options()
  .descr("Validate one direct chrono archive and emit canonical rkyv 0.8 bytes")
  .command("migrate")
  .to_options()
  .version(env!("CARGO_PKG_VERSION"))
}

/// Executes the parsed command.
fn execute(options: Options) -> Result<(), CliError> {
  execute_migrate(
    &options.archive_type,
    &options.source_version,
    &options.input,
    options.output,
    options.in_place,
    options.replace,
  )
}

/// Validates command options, performs migration, and writes only successful
/// canonical output.
fn execute_migrate(
  archive_type: &str,
  source_version: &str,
  input: &Path,
  output: Option<PathBuf>,
  in_place: bool,
  replace: bool,
) -> Result<(), CliError> {
  let destination = match (output, in_place) {
    (Some(output), false) => Destination::Output(output),
    (None, true) => Destination::InPlace,
    (Some(_), true) => {
      return Err(CliError::InvalidOptions(
        "`--output` and `--in-place` are mutually exclusive".to_owned(),
      ));
    }
    (None, false) => {
      return Err(CliError::InvalidOptions(
        "one of `--output` or `--in-place` is required".to_owned(),
      ));
    }
  };
  if replace && matches!(destination, Destination::InPlace) {
    return Err(CliError::InvalidOptions(
      "`--replace` applies only to a separate `--output` path".to_owned(),
    ));
  }

  let hint = parse_source_hint(source_version)?;
  let bytes = fs::read(input).map_err(|source| CliError::Read {
    path: input.to_owned(),
    source,
  })?;
  let canonical = migrate_direct(archive_type, &bytes, hint)?;
  let source = chrono_rkyv_migrate::source_format();
  let target = chrono_rkyv_migrate::target_format();
  eprintln!(
    "source mode: {}; target format: {}",
    display_format(source),
    display_format(target)
  );

  match destination {
    Destination::Output(output) => write_output(&output, &canonical, replace),
    Destination::InPlace => replace_in_place(input, &canonical),
  }
}

/// Parses the stable source-version vocabulary.
fn parse_source_hint(value: &str) -> Result<SourceHint, CliError> {
  match value {
    "auto" => Ok(SourceHint::Auto),
    "0.7" => Ok(SourceHint::Rkyv0_7),
    "0.8" => Ok(SourceHint::Rkyv0_8),
    _ => Err(CliError::InvalidOptions(format!(
      "unsupported source version `{value}`; expected `auto`, `0.7`, or `0.8`"
    ))),
  }
}

/// Migrates one CLI-supported direct chrono root value.
#[allow(
  deprecated,
  reason = "the explicitly supported legacy Date CLI tokens preserve access to historical archives"
)]
fn migrate_direct(
  archive_type: &str,
  bytes: &[u8],
  hint: SourceHint,
) -> Result<rkyv::util::AlignedVec, MigrationError> {
  macro_rules! migrate_bytes {
    ($target:ty) => {
      chrono_rkyv_migrate::migrate::<$target>(bytes, hint)
        .map(|migrated| migrated.canonical_bytes)
    };
  }

  match archive_type {
    "time-delta" | "duration" => migrate_bytes!(chrono::TimeDelta),
    "naive-date" => migrate_bytes!(chrono::NaiveDate),
    "naive-time" => migrate_bytes!(chrono::NaiveTime),
    "naive-datetime" => migrate_bytes!(chrono::NaiveDateTime),
    "iso-week" => migrate_bytes!(chrono::IsoWeek),
    "utc" => migrate_bytes!(chrono::Utc),
    "fixed-offset" => migrate_bytes!(chrono::FixedOffset),
    "local" => migrate_bytes!(chrono::Local),
    "month" => migrate_bytes!(chrono::Month),
    "weekday" => migrate_bytes!(chrono::Weekday),
    "datetime-utc" => migrate_bytes!(chrono::DateTime<chrono::Utc>),
    "datetime-fixed-offset" => {
      migrate_bytes!(chrono::DateTime<chrono::FixedOffset>)
    }
    "datetime-local" => migrate_bytes!(chrono::DateTime<chrono::Local>),
    "date-utc" => migrate_bytes!(chrono::Date<chrono::Utc>),
    "date-fixed-offset" => migrate_bytes!(chrono::Date<chrono::FixedOffset>),
    "date-local" => migrate_bytes!(chrono::Date<chrono::Local>),
    _ => Err(MigrationError::UnsupportedDirectType {
      type_name: archive_type.to_owned(),
    }),
  }
}

/// Writes a separate output with create-new behavior unless replacement was
/// explicitly selected.
fn write_output(path: &Path, bytes: &[u8], replace: bool) -> Result<(), CliError> {
  let mut options = OpenOptions::new();
  options.write(true);
  if replace {
    options.create(true).truncate(true);
  } else {
    options.create_new(true);
  }
  let file = options.open(path).map_err(|source| CliError::Write {
    path: path.to_owned(),
    source,
  })?;
  write_and_sync(file, path, bytes)
}

/// Writes and syncs all bytes before considering the destination successful.
fn write_and_sync(mut file: File, path: &Path, bytes: &[u8]) -> Result<(), CliError> {
  file.write_all(bytes)
    .and_then(|()| file.sync_all())
    .map_err(|source| CliError::Write {
      path: path.to_owned(),
      source,
    })
}

/// Writes a sibling temporary file and atomically renames it over the input.
fn replace_in_place(input: &Path, bytes: &[u8]) -> Result<(), CliError> {
  let parent = input.parent().unwrap_or_else(|| Path::new("."));
  let file_name = input.file_name().ok_or_else(|| {
    CliError::InvalidOptions(format!(
      "input path `{}` has no file name",
      input.display()
    ))
  })?;
  let permissions = fs::metadata(input)
    .map_err(|source| CliError::Read {
      path: input.to_owned(),
      source,
    })?
    .permissions();

  let mut last_collision = None;
  for attempt in 0..128_u16 {
    let temporary = temporary_path(parent, file_name, attempt);
    match OpenOptions::new().write(true).create_new(true).open(&temporary) {
      Ok(mut file) => {
        let prepared = file
          .write_all(bytes)
          .and_then(|()| file.set_permissions(permissions.clone()))
          .and_then(|()| file.sync_all());
        if let Err(source) = prepared {
          let _cleanup = fs::remove_file(&temporary);
          return Err(CliError::Write {
            path: temporary,
            source,
          });
        }
        drop(file);
        if let Err(source) = fs::rename(&temporary, input) {
          let _cleanup = fs::remove_file(&temporary);
          return Err(CliError::Replace {
            path: input.to_owned(),
            source,
          });
        }
        return Ok(());
      }
      Err(source) if source.kind() == io::ErrorKind::AlreadyExists => {
        last_collision = Some(source);
      }
      Err(source) => {
        return Err(CliError::Write {
          path: temporary,
          source,
        });
      }
    }
  }

  Err(CliError::Write {
    path: input.to_owned(),
    source: last_collision.unwrap_or_else(|| {
      io::Error::new(
        io::ErrorKind::AlreadyExists,
        "could not allocate a sibling temporary file",
      )
    }),
  })
}

/// Produces one collision-resistant sibling temporary path.
fn temporary_path(parent: &Path, file_name: &std::ffi::OsStr, attempt: u16) -> PathBuf {
  let mut temporary_name = OsString::from(".");
  temporary_name.push(file_name);
  temporary_name.push(format!(
    ".chrono-rkyv-migrate-{}-{attempt}.tmp",
    std::process::id()
  ));
  parent.join(temporary_name)
}

/// Formats representation facts for the pre-write diagnostic.
fn display_format(format: ArchiveFormat) -> String {
  let version = match format.version {
    chrono_rkyv_migrate::ArchiveVersion::Rkyv0_7 => "rkyv 0.7",
    chrono_rkyv_migrate::ArchiveVersion::Rkyv0_8 => "rkyv 0.8",
  };
  let width = match format.width {
    PointerWidth::Bits16 => "16-bit",
    PointerWidth::Bits32 => "32-bit",
    PointerWidth::Bits64 => "64-bit",
  };
  let endianness = match format.endianness {
    Endianness::Little => "little-endian",
    Endianness::Big => "big-endian",
  };
  format!("{version}, {width}, {endianness}")
}

#[cfg(test)]
mod tests {
  use std::fs;

  use super::{
    execute_migrate, migrate_direct, options, parse_source_hint, replace_in_place,
    write_output, CliError, SourceHint,
  };
  use strict_test_support::{
    ensure, ensure_ok, TempDir, TestFailure,
  };

  #[test]
  fn parses_source_versions() -> Result<(), TestFailure> {
    ensure(
      ensure_ok(parse_source_hint("auto"), "parse auto source hint")? == SourceHint::Auto,
      "auto maps to automatic source detection",
    )?;
    ensure(
      ensure_ok(parse_source_hint("0.7"), "parse legacy source hint")? == SourceHint::Rkyv0_7,
      "0.7 maps to the legacy source",
    )?;
    ensure(
      ensure_ok(parse_source_hint("0.8"), "parse current source hint")? == SourceHint::Rkyv0_8,
      "0.8 maps to the current source",
    )?;
    ensure(
      parse_source_hint("1").is_err(),
      "unknown source versions are rejected",
    )
  }

  #[test]
  fn bpaf_parser_accepts_the_documented_migrate_surface() -> Result<(), TestFailure> {
    let parsed = match options().run_inner(&[
        "migrate",
        "--type",
        "naive-date",
        "--source-version",
        "auto",
        "--input",
        "input.bin",
        "--output",
        "output.bin",
        "--replace",
      ]) {
      Ok(parsed) => parsed,
      Err(_) => {
        return Err(TestFailure::Condition {
          context: "documented migrate command parses",
        });
      }
    };
    ensure(
      parsed.archive_type == "naive-date",
      "parser retains the direct chrono type",
    )?;
    ensure(
      parsed.source_version == "auto",
      "parser retains the source hint",
    )?;
    ensure(
      parsed.input == std::path::Path::new("input.bin"),
      "parser retains the input path",
    )?;
    ensure(
      parsed.output.as_deref() == Some(std::path::Path::new("output.bin")),
      "parser retains the output path",
    )?;
    ensure(!parsed.in_place, "output mode does not select in-place")?;
    ensure(parsed.replace, "parser retains explicit replacement")
  }

  #[test]
  fn destination_selection_rejects_ambiguous_options() -> Result<(), TestFailure> {
    ensure(
      matches!(
        execute_migrate(
          "month",
          "0.8",
          std::path::Path::new("missing.bin"),
          Some(std::path::PathBuf::from("output.bin")),
          true,
          false,
        ),
        Err(CliError::InvalidOptions(_))
      ),
      "output and in-place are mutually exclusive",
    )?;
    ensure(
      matches!(
        execute_migrate(
          "month",
          "0.8",
          std::path::Path::new("missing.bin"),
          None,
          false,
          false,
        ),
        Err(CliError::InvalidOptions(_))
      ),
      "one destination is required",
    )?;
    ensure(
      matches!(
        execute_migrate(
          "month",
          "0.8",
          std::path::Path::new("missing.bin"),
          None,
          true,
          true,
        ),
        Err(CliError::InvalidOptions(_))
      ),
      "replace is rejected for in-place mode",
    )
  }

  #[test]
  fn separate_output_requires_explicit_replacement() -> Result<(), TestFailure> {
    let directory = TempDir::new("chrono-rkyv-migrate-replace")?;
    let output = directory.path().join("archive.bin");
    fs::write(&output, b"old")?;

    ensure(
      matches!(
      write_output(&output, b"new", false),
      Err(CliError::Write { .. })
      ),
      "create-new output refuses an existing path",
    )?;
    ensure(
      fs::read(&output)? == b"old",
      "replacement refusal preserves existing bytes",
    )?;
    ensure_ok(
      write_output(&output, b"new", true),
      "explicitly replace output",
    )?;
    ensure(
      fs::read(&output)? == b"new",
      "explicit replacement writes new bytes",
    )
  }

  #[test]
  fn in_place_replacement_writes_complete_new_bytes() -> Result<(), TestFailure> {
    let directory = TempDir::new("chrono-rkyv-migrate-in-place")?;
    let input = directory.path().join("archive.bin");
    fs::write(&input, b"old")?;

    ensure_ok(
      replace_in_place(&input, b"canonical"),
      "replace archive in place",
    )?;
    ensure(
      fs::read(&input)? == b"canonical",
      "in-place replacement writes complete canonical bytes",
    )
  }

  #[test]
  fn full_migrate_command_writes_decodable_current_bytes() -> Result<(), TestFailure> {
    let directory = TempDir::new("chrono-rkyv-migrate-command")?;
    let input = directory.path().join("legacy.bin");
    let output = directory.path().join("current.bin");
    let current = ensure_ok(
      rkyv::to_bytes::<rancor::Error>(&chrono::Month::March),
      "encode current month input",
    )?;
    fs::write(&input, current)?;

    ensure_ok(
      execute_migrate("month", "0.8", &input, Some(output.clone()), false, false),
      "execute direct month migration",
    )?;
    let migrated = fs::read(output)?;
    let decoded = ensure_ok(
      rkyv::from_bytes::<chrono::Month, rancor::Error>(&migrated),
      "decode CLI output as current month",
    )?;
    ensure(
      decoded == chrono::Month::March,
      "CLI output preserves the direct chrono value",
    )
  }

  #[test]
  fn failed_validation_preserves_in_place_input() -> Result<(), TestFailure> {
    let directory = TempDir::new("chrono-rkyv-migrate-preserve")?;
    let input = directory.path().join("archive.bin");
    let original = [u8::MAX];
    fs::write(&input, original)?;

    ensure(
      matches!(
        execute_migrate("month", "0.8", &input, None, true, false),
        Err(CliError::Migration(_))
      ),
      "invalid archive fails before in-place replacement",
    )?;
    ensure(
      fs::read(&input)? == original,
      "failed migration preserves the original input",
    )?;
    ensure(
      fs::read_dir(directory.path())?.count() == 1,
      "failed migration leaves no sibling temporary file",
    )
  }

  #[test]
  fn unsupported_application_root_type_is_typed() -> Result<(), TestFailure> {
    ensure(
      matches!(
        migrate_direct("application-container", &[], SourceHint::Auto),
        Err(chrono_rkyv_migrate::MigrationError::UnsupportedDirectType { .. })
      ),
      "application-owned schemas are rejected by the direct CLI",
    )
  }
}
