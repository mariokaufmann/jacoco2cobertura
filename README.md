# Jacoco2Cobertura

Small utility to convert a Jacoco coverage report to a Cobertura coverage report. This can for example be useful when
working with Gitlab CI, which does not support JaCoCo reports yet (
see [issue](https://gitlab.com/gitlab-org/gitlab/-/issues/227345)).

## Usage

Download the newest release version from
the [releases page](https://github.com/mariokaufmann/jacoco2cobertura/releases/latest). Then run the utility directly:

```bash
./jacoco2cobertura --input-file jacoco.xml --output-file cobertura.xml
```

To find out all available options use the `--help` option:

```bash
./jacoco2cobertura --help
```

## Credit

This repository is an (incomplete) port of [cover2cover](https://github.com/rix0rrr/cover2cover).
