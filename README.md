# Programowanie w języku Rust - Szkolenie

## Docs

* https://doc.rust-lang.org/book/
* https://doc.rust-lang.org/std/index.html

## Konfiguracja środowiska

Proszę wybrać jedną z poniższych opcji:

### Lokalna

Przed szkoleniem należy zainstalować:

* Rust - https://www.rust-lang.org/tools/install
* [Visual Studio Code](https://code.visualstudio.com/)
  * Zainstalować wtyczki
    * rust-analyzer
    * Jupyter
    * Live Share
* Rust REPL - `evcxr_repl`

  ```bash
  cargo install evcxr_repl
  cargo install evcxr_jupyter
  evcxr_jupyter --install
  ```


### Docker + Visual Studio Code

Jeśli uczestnicy szkolenia korzystają w pracy z Docker'a, to należy zainstalować:

#### Docker

* Instalacja Docker Desktop - https://www.docker.com/get-started/

#### Visual Studio Code

* [Visual Studio Code](https://code.visualstudio.com/)
* Zainstalować wtyczki
  * Live Share
  * Dev Containers ([wymagania](https://code.visualstudio.com/docs/devcontainers/containers#_system-requirements))
    * po instalacji wtyczki - należy otworzyć w VS Code folder zawierający sklonowane repozytorium i
      z palety poleceń (Ctrl+Shift+P) wybrać opcję **Dev Containers: Rebuild and Reopen in Container**

