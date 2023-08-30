<div align="center" id="top">
  <img src="./.github/logo-jenkenpwn.jpg" width=50% alt="Jenkenpwn" />

  &#xa0;

  <!-- <a href="https://jenkenpwn.netlify.app">Demo</a> -->
</div>

<h1 align="center">Jenkenpwn</h1>

<p align="center">
  <img alt="Github top language" src="https://img.shields.io/github/languages/top/y0k4i-1337/jenkenpwn?color=56BEB8">

  <img alt="Github language count" src="https://img.shields.io/github/languages/count/y0k4i-1337/jenkenpwn?color=56BEB8">

  <img alt="Repository size" src="https://img.shields.io/github/repo-size/y0k4i-1337/jenkenpwn?color=56BEB8">

  <img alt="License" src="https://img.shields.io/github/license/y0k4i-1337/jenkenpwn?color=56BEB8">

  <!-- <img alt="Github issues" src="https://img.shields.io/github/issues/y0k4i-1337/jenkenpwn?color=56BEB8" /> -->

  <!-- <img alt="Github forks" src="https://img.shields.io/github/forks/y0k4i-1337/jenkenpwn?color=56BEB8" /> -->

  <!-- <img alt="Github stars" src="https://img.shields.io/github/stars/y0k4i-1337/jenkenpwn?color=56BEB8" /> -->
</p>

<!-- Status -->

<!-- <h4 align="center">
	🚧  Jenkenpwn 🚀 Under construction...  🚧
</h4>

<hr> -->

<p align="center">
  <a href="#dart-about">About</a> &#xa0; | &#xa0;
  <a href="#sparkles-features">Features</a> &#xa0; | &#xa0;
  <a href="#rocket-technologies">Technologies</a> &#xa0; | &#xa0;
  <a href="#white_check_mark-requirements">Requirements</a> &#xa0; | &#xa0;
  <a href="#checkered_flag-starting">Starting</a> &#xa0; | &#xa0;
  <a href="#memo-license">License</a> &#xa0; | &#xa0;
  <a href="https://github.com/y0k4i-1337" target="_blank">Author</a>
</p>

<br>

## :dart: About ##

`Jenkenpwn` is a Rust application that enables users to extract data from a
Jenkins server through its native API. It supports retrieving jobs
information as well as builds details.

## :sparkles: Features ##

:heavy_check_mark: Retrieve detailed job information from a Jenkins server;\
:heavy_check_mark: Access build details such as console output and  injected
environment variables, which may leak sensitive information;\
:heavy_check_mark: Asynchronous operations using the Tokio runtime for improved performance.

## :rocket: Technologies ##

`Jenkenpwn` is built using modern Rust programming practices and leverages the following libraries and technologies:

- [Tokio](https://tokio.rs/) - A runtime for writing asynchronous Rust applications.
- [Reqwest](https://docs.rs/reqwest/) - A Rust HTTP client for making requests.

## :white_check_mark: Requirements ##

Before starting :checkered_flag:, you need to have [Git](https://git-scm.com) and [Rust](https://rust-lang.org/) installed.

## :checkered_flag: Starting ##

```bash
# Clone this project
$ git clone https://github.com/y0k4i-1337/jenkenpwn

# Access
$ cd jenkenpwn

# Install dependencies and build project
$ cargo build --release

# Run the project
$ ./target/release/jenkenpwn -h
```

### Main Help Menu ###

```
./target/release/jenkenpwn -h
Jenkins pwning tool FTW

Usage: jenkenpwn [OPTIONS] <COMMAND>

Commands:
  dump  Dump jobs and builds data
  help  Print this message or the help of the given subcommand(s)

Options:
  -v, --verbose   Verbose mode
  -i, --insecure  Do not verify SSL certificate
  -h, --help      Print help
  -V, --version   Print version
```

#### Dump Sub-command Help Menu ####

```
./target/release/jenkenpwn dump -h
Dump jobs and builds data

Usage: jenkenpwn dump [OPTIONS] <RESOURCE> <URL>

Arguments:
  <RESOURCE>  Resources to dump [possible values: builds, jobs, views]
  <URL>       Url of the jenkins server

Options:
  -u, --username <USERNAME>  Username for authentication
  -p, --password <PASSWORD>  Password for authentication
  -r, --recover              Recover from server failure, skiping already downloaded builds
  -o, --output <OUTPUT>      Output directory [default: dumps]
  -l, --last                 Dump only the last build of each job
  -j, --jobs <JOBS>          Read jobs from a jobs dump file
  -h, --help                 Print help (see more with '--help')
  -V, --version              Print version
```

## :memo: License ##

This project is under license from MIT. For more details, see the [LICENSE](LICENSE.md) file.


Made with :heart: by <a href="https://github.com/y0k4i-1337" target="_blank">y0k4i</a>

&#xa0;

<a href="#top">Back to top</a>
