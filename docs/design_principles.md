# Design Principles

This document aims to provide some insight on the design principles I employed while developing the binaries/libraries involved in this project.

## `pidgtm`

In StartGG, gamer tag's are not unique, so, to uniquely identify users, the platform uses [slugs](https://developer.mozilla.org/en-US/docs/Glossary/Slug) or player IDs. Due to this, it is not possible to search for a player provided only their gamer tag. As I wanted to avoid an experience that relied on players having to grab slugs to be able use the tool, it became evident that I needed to create a mapping between slugs, player ids, and gamer tags.

At this point, I had two options:
1) utilize [the player database](https://github.com/smashdata/ThePlayerDatabase) provided by the smashdata project, or
2) create my own mapping.

I've opted for the second option to avoid heavily relying on an outside project for my tool's most basic needs.

### PostgreSQL, Diesel, and Migrations

`pidgtm` is powered by PostgreSQL (i.e., SQL flavour), and [Diesel](https://diesel.rs/) (an [object relational mapping (ORM)](https://en.wikipedia.org/wiki/Object%E2%80%93relational_mapping)). With Diesel, we can set up migrations, which an utility that allows us to granually evolve our database over time — every migration can be applied (i.e., with its' respective `up.sql`) or reverted (i.e., with its' respective `down.sql`).

