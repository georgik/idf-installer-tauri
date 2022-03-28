# idf-installer 3 (preview)

## Project setup
```
yarn install
```

### Compiles and hot-reloads for development
```
yarn serve
```

### Compiles and minifies for production
```
yarn build
```

### Lints and fixes files
```
yarn lint
```

### Customize configuration
See [Configuration Reference](https://cli.vuejs.org/config/).


## Feature parity

### Features required to match version 2.x

- build signed binary
- check network connectivity
- deploy embedded Python
- deploy embedded Git
- deploy drivers
- deploy JDK
- deploy Espressif-IDE
- deploy Rust
- deploy Toit
- use Gitee mirror for cloning
- use assets mirror
- L10N
- check for incorrect paths (spaces, special characters)
- Windows Terminal link generator
- Desktop links
- Windows Defender exclusions
- Offline installation

### New features to be implemented

- observe state of current deployment and update UI
- removing installed toolchain
- deploy Qemu
- link to Product Selector
- deploy DevContainers
- open terminal with activated shell
- notification area - inform user what's new
- auto-updater
- verify integrity of downloaded packages
- check for antivirus, warn when running in sandbox
- check whether it's possible to write to target location
- support for manually create repos
- installation via choco
- check for new releases
- support for constraints files
- verify integrity of CMD
