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

If you want to pass source roots (i.e. the absolute path to the folders in which the sources file mentioned in the report are located), you can use the argument `--source-root` for that:
```bash
./jacoco2cobertura --input-file jacoco.xml --output-file cobertura.xml --source-root module1/src/main/java --source-root module2/src/main/java
```

To find out all available options use the `--help` option:

```bash
./jacoco2cobertura --help
```

## Credit

This repository is a port of [cover2cover](https://github.com/rix0rrr/cover2cover).
