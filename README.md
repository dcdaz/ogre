# OGRE

**Ogre** is a small CLI tool to sync files with a *WebDav* server. The idea came out because I'm using [Orglzy Revived](https://www.orgzlyrevived.com/) for taking notes and add some tasks, but I didn't have a good way to sync them with any of my laptops and re-file, review or edit such notes/tasks.

## TL;DR

I didn't want to use [Cadaver](https://github.com/notroj/cadaver) to sync anything nor Rsync or any other tool and took the oportunity to improve a bit my *Rust* skills.

## Run

```bash
ogre
```

> **Ogre** will prompt you for username and password of your *WebDav* server in 2 different ocasions
> - Credentials file doesn't exists 
> - Crendentials file is empty.

### Debug Mode

It can enable *Debug* mode to print extra information with **env** variable **OGRE_DEBUG=true**

```bash
OGRE_DEBUG=true ogre
```

## Config

Config is a YAML file called `ogre.yml` stored on your `config` dir, like `/home/{user}/.config/ogre.yml`, and content is like the following:

```yml
hostname: https://webdavinstance.dav
remote_path: /remote.php/dav/files/{user}/Documents/syncable
local_path: /home/{user}/Documents/syncable
```

## Cache

**Ogre** uses a cache file called `ogre` stored on your `cache` dir, like `/home/{user}/.cache/ogre` to store credentials encoded with **Base64** *(I know, not the safest way to do it and I'll improve that in the future)*.

## TODO

- [x] Sync existing files on WebDav server
- [ ] Expires Credentials after X days/hours
- [ ] Add arguments so it can create files, remove files, create folders, remove folders
- [ ] Add Unit Tests
- [ ] Add a better encoding for credentials or cypher them
- [ ] Add daemon mode so it can run in background and auto-sync every X minutes
- [ ] Recover from corrupted credentials file, either clean it or remove it