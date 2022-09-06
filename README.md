<p align="center">
  <a href="" rel="noopener">
 <img width=200px height=200px src="https://i.imgur.com/6wj0hh6.jpg" alt="Project logo"></a>
</p>

<h3 align="center">OnlyCringe !</h3>

<div align="center">

[![Status](https://img.shields.io/badge/status-active-success.svg)]()
[![GitHub Issues](https://img.shields.io/github/issues/kylelobo/The-Documentation-Compendium.svg)](https://github.com/kylelobo/The-Documentation-Compendium/issues)
[![GitHub Pull Requests](https://img.shields.io/github/issues-pr/kylelobo/The-Documentation-Compendium.svg)](https://github.com/kylelobo/The-Documentation-Compendium/pulls)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](/LICENSE)

</div>

---

<p align="center"> The freest social network EVER
    <br> 
</p>

## 📝 Table of Contents

- [About](#about)
- [Getting Started](#getting_started)
- [Deployment](#deployment)
- [Usage](#usage)
- [TODO](../TODO.md)
- [Contributing](../CONTRIBUTING.md)
- [Authors](#authors)
- [Acknowledgments](#acknowledgement)

## 🧐 About <a name = "about"></a>

This project is a social network that I want to create for testing my web-developping skills, I think realising a social network is the best things to test your capacities because it have many features to implements and many front end to create (I don't like this steps...)

## 🏁 Getting Started <a name = "getting_started"></a>

- Download the repository by:
```
git clone git@github.com:pulk66-s/OnlyCringe.git
```
- choose your version (folder at the project root)
- please install projects dependecies (depending on which version you are choosing)

### Prerequisites

Prerequisites are dependings of the version

- RustVue
```
- rustc
- rust Rocket
- mariadb (NOT mysql it's important)
- vuejs

For dockerizing:
- docker
- docker-compose

For testing:
- python behave
- python selenium
```

### Installing

Installing depends on the version you are choosing but in each version (if I didn't forgot) you can found scripts to install.

But globally it will be scripts like this:
```
- You need to be on frontend or backend folder
./scripts/install.sh
(or docker)
./scripts/docker/install.sh
```

## 🔧 Running the tests <a name = "tests"></a>

Testing depends on the version you are choosing but in each version (if I didn't forgot) you can found scripts to tests.

But globally it will be scripts like this:
```
- You need to be on frontend or backend folder
./scripts/tests/launch.sh
```
If you don't want to launch all the tests you can found others scripts in ./scripts/tests

### And coding style tests

Coding styles are a major problem to keep your code clean. If you want to, just learn the basic coding style for the language you are using but there is some constants:
- choose between snake_case and camelCase and KEEP it
- classes are always in UpperSnakeCase
- 1 file = 1 utilities
and other things but I'm lazy so just know them

## 🎈 Usage <a name="usage"></a>

Usage depends on the version you are choosing but in each version (if I didn't forgot) you can found scripts to tests.

But globally it will be scripts like this:
```
./scripts/launch/all.sh
(or docker)
./scripts/docker/launch.sh
```

## 🚀 Deployment <a name = "deployment"></a>

Deployment are in a CICD on production branch, just push on it

## ✍️ Authors <a name = "authors"></a>

- [@pulk66-s](https://github.com/pulk66-s) - Initial Idea and developer
- [@theobori](https://github.com/theobori) - Developer

## 🎉 Acknowledgements <a name = "acknowledgement"></a>

- Just for fun don't be serious
