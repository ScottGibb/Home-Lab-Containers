# Changelog

## [3.0.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v2.2.0...v3.0.0) (2026-04-26)


### ⚠ BREAKING CHANGES

* add PiHall

### Features

* add docker compose plugin for linting ([6f1d4dc](https://github.com/ScottGibb/Home-Lab-Containers/commit/6f1d4dc986053ddf71f8da184f82fb0f4e59a57e))
* add fstab for PiHall ([2563967](https://github.com/ScottGibb/Home-Lab-Containers/commit/256396734712d74d4cfb3e53436eeb9420cb938d))
* add missing nginx.conf ([d42e247](https://github.com/ScottGibb/Home-Lab-Containers/commit/d42e2479d83ff1ccad2ebdf0255560e07c6b2620))
* add PiHall ([b6d5b46](https://github.com/ScottGibb/Home-Lab-Containers/commit/b6d5b46b233b95683df30a15c7b9eb76dbc4f6d1))
* add vscode settings for formatters ([b16d135](https://github.com/ScottGibb/Home-Lab-Containers/commit/b16d1353c5064858b52c0d8bbd762ae7b50d2b56))


### Bug Fixes

* config folder ([58ce8c2](https://github.com/ScottGibb/Home-Lab-Containers/commit/58ce8c28c166f7655804b9683cf9dcd80c369ed0))
* dclint linting errors: add interface prefixes to ports and concrete image version tags ([2b9785e](https://github.com/ScottGibb/Home-Lab-Containers/commit/2b9785e536452e1968cf0954ad44db9ba26c96f0))
* dependabot submodule issues ([58f32a9](https://github.com/ScottGibb/Home-Lab-Containers/commit/58f32a99c7096d85ac0ba8fb6923d98a8dc9681c))
* Docker compose format ([c30398f](https://github.com/ScottGibb/Home-Lab-Containers/commit/c30398fb7f8d3b85e38bf63b39e3ea482581c01c))
* **docs:** Dependabot URL configuration in README ([881e085](https://github.com/ScottGibb/Home-Lab-Containers/commit/881e085ca39fdecaff904063bfc36bcd47fb58f1))
* folder paths ([86f77bc](https://github.com/ScottGibb/Home-Lab-Containers/commit/86f77bc8cf0a1a19a298632d29d5d397c11ab4da))
* networking on pihome ([60ab9a5](https://github.com/ScottGibb/Home-Lab-Containers/commit/60ab9a54cde3ad1f0823d9424cdf28b4b327e39b))
* pihall dns ([335ea88](https://github.com/ScottGibb/Home-Lab-Containers/commit/335ea889652971f1724cfb667544ee2a850d81e7))
* Pinning Dockerfile versions ([21fe9a4](https://github.com/ScottGibb/Home-Lab-Containers/commit/21fe9a4fedc46b1abeb7da40467d553be3d857f5))
* remove bind mount ([9a30e16](https://github.com/ScottGibb/Home-Lab-Containers/commit/9a30e162e49bcf76e3b9a4851b6d0c44b69d3bca))
* rm ftstab ([0faebdb](https://github.com/ScottGibb/Home-Lab-Containers/commit/0faebdb3d8d1ff53152f658a91010abdae7831d4))
* tagged versions for docker compose ([f14c549](https://github.com/ScottGibb/Home-Lab-Containers/commit/f14c5492e6287d0ee5458df185e416f8d4974f99))
* Update dependabot.yaml to remove directories ([cfcae3d](https://github.com/ScottGibb/Home-Lab-Containers/commit/cfcae3dd36dfb2ee0d5dc3fc05c07b466252fffe))
* wifi connect removal ([e5baa3c](https://github.com/ScottGibb/Home-Lab-Containers/commit/e5baa3c65c608bdbbdab2d3ca6232d0c92a78b9a))

## [2.2.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v2.1.1...v2.2.0) (2026-02-18)

### Features

* Add name attribute to all subdirectory docker-compose files ([a587178](https://github.com/ScottGibb/Home-Lab-Containers/commit/a58717875e7eb00ad8c4b381346b6d96581e4c7b))
* Create main docker-compose files with include directives and inline env vars ([a40643b](https://github.com/ScottGibb/Home-Lab-Containers/commit/a40643b98a8d2bfe3a0cf56c7602c0f50d92d761))
* migrate to one nginx ([c861adf](https://github.com/ScottGibb/Home-Lab-Containers/commit/c861adf5d0bb2a13482fdc2c116a320573b2ed8b))
* Remove all bash scripts and .env files ([e958e30](https://github.com/ScottGibb/Home-Lab-Containers/commit/e958e30f4d801da6afeba632e5822dba418eee58))

### Bug Fixes

* add nas stack ([b28a81d](https://github.com/ScottGibb/Home-Lab-Containers/commit/b28a81d3cfb57d4a62265469a9b76f7bd3f2e443))
* case sensitivity in Dependabot exclude paths for private submodule ([f555f7b](https://github.com/ScottGibb/Home-Lab-Containers/commit/f555f7b0b5acd99163ad70195094c7bf15d0198e))
* **docs:** README ([6dac1e7](https://github.com/ScottGibb/Home-Lab-Containers/commit/6dac1e75f8e4c0984c6e8f3f609f4c7c568e1a11))
* Gitlab version ([43897d3](https://github.com/ScottGibb/Home-Lab-Containers/commit/43897d3c2b94fe71a2a7fd3030c34a2123a0976f))
* make portainer data persistent ([0ec9f03](https://github.com/ScottGibb/Home-Lab-Containers/commit/0ec9f038bfc5a461d2a7d3f99c99c8850d16e5d7))

## [2.1.1](https://github.com/ScottGibb/Home-Lab-Containers/compare/v2.1.0...v2.1.1) (2026-02-08)

### Bug Fixes

* nginx ip ([f067775](https://github.com/ScottGibb/Home-Lab-Containers/commit/f06777531c43e3280fdf7d85fbc4e8babbe59524))
* pin nginx ([8d93c3a](https://github.com/ScottGibb/Home-Lab-Containers/commit/8d93c3aee92f8fdc8a4ce6c10e40acb52aabdd83))

## [2.1.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v2.0.0...v2.1.0) (2026-02-08)

### Features

* add pinned utils ([dcdfde0](https://github.com/ScottGibb/Home-Lab-Containers/commit/dcdfde03d0b80845646c937e8aa5c14832b31d6b))

### Bug Fixes

* use pinned image for gitlab ([dc7700a](https://github.com/ScottGibb/Home-Lab-Containers/commit/dc7700a1607fa1b33e442ddea306fbfe00f9335e))

## [2.0.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v1.5.0...v2.0.0) (2026-02-08)

### ⚠ BREAKING CHANGES

* removing piweb

### Features

* Add Docker Compose to Dependabot configuration ([f436bfa](https://github.com/ScottGibb/Home-Lab-Containers/commit/f436bfa0ce2c3d2f005dd5eabb10b14b3caba07d))
* bump deps ([e21ac93](https://github.com/ScottGibb/Home-Lab-Containers/commit/e21ac93688672964a765bf4b64a0a300870e6a8f))
* removing piweb ([b1a0f30](https://github.com/ScottGibb/Home-Lab-Containers/commit/b1a0f3023a9ad6f03c597a4964d8626ae11c6331))
* use prebuilt image for presence detector ([e43103b](https://github.com/ScottGibb/Home-Lab-Containers/commit/e43103bad0c76dcba48932be4625e825a729d0fa))

### Bug Fixes

* dependabot ([993f0f9](https://github.com/ScottGibb/Home-Lab-Containers/commit/993f0f97be16c60e4e18701c7dcccbe80a909d4d))
* docker path for smart rf transmitter ([23916b9](https://github.com/ScottGibb/Home-Lab-Containers/commit/23916b9b3ee3c3ea0454d4ded80be52707b43aa9))
* **docs:** naming conventions ([37e862d](https://github.com/ScottGibb/Home-Lab-Containers/commit/37e862d42bd5970370f0413f6b137a38e59d1abd))
* **docs:** remove png ([150649c](https://github.com/ScottGibb/Home-Lab-Containers/commit/150649cdd51e915a9c0659d4a4de55e8787451fb))
* tty issues with cmdline tui ([afeb7c2](https://github.com/ScottGibb/Home-Lab-Containers/commit/afeb7c227cbe23b597f6528686fe5c0092d41393))

## [1.5.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v1.4.0...v1.5.0) (2025-12-15)

### Features

* **ci:** Update Dependabot workflow for auto-merging ([a5f691f](https://github.com/ScottGibb/Home-Lab-Containers/commit/a5f691fa971a8ab964d132e02d44d61aa865faf4))

### Bug Fixes

* **ci:** Add directory scope to Dependabot ([e371e89](https://github.com/ScottGibb/Home-Lab-Containers/commit/e371e89a8e58d6056e484bdef2eaa58b80393beb))

## [1.4.0](https://github.com/ScottGibb/Home-Lab-Containers/compare/v1.3.2...v1.4.0) (2025-12-02)

### Features

* Remove gitsubmodule configuration from dependabot ([ff35c81](https://github.com/ScottGibb/Home-Lab-Containers/commit/ff35c81247846c3d9ba8a8a238d1922dab837b05))

### Bug Fixes

* **ci:** Add Dependabot configuration ([701af8b](https://github.com/ScottGibb/Home-Lab-Containers/commit/701af8b39bc8cfd6e901a247103f9444557551b7))
* **ci:** Dependabot ([b351ab4](https://github.com/ScottGibb/Home-Lab-Containers/commit/b351ab4fed84e57a46fdc400b925cb273f6793fa))
* **ci:** fix dependabot again ([27e45ae](https://github.com/ScottGibb/Home-Lab-Containers/commit/27e45aea47cb0d9bf48eb56b5a9a79ab39d33ca4))

## [1.3.2](https://github.com/ScottGibb/Home-Lab-Containers/compare/v1.3.1...v1.3.2) (2025-11-30)

### Bug Fixes

* **ci:** fix dependabot naming ([be38fed](https://github.com/ScottGibb/Home-Lab-Containers/commit/be38fed4d5f82b02cbd71e8f1dc9b7d6d11be27c))
