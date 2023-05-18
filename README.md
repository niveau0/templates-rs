# Rust templates

This repository contains some (or at least should contain in future) simple opinionated templates to start a Rust project, all following domain centric architecture principles.

## Usage

To generate a project from a given template use cargo-generate. See command for templates below.

## Existing templates

* Command line interface parsing skeleton, using clap crate and a simple command pattern.

        cargo generate -g niveau0/templates-rs cli-rs

## Features

* create template with async, using tokio or async_std runtime, or sync code skeleton.

## Planned

* web service skeleton with a REST API
    * optional WASM UI with tailwind css and trunk for bundling
    * optional DB connection
