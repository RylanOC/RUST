#!/usr/bin/env bash
# Nia Calia-Bogan
# Settings and environment variables.
# run before launching the RUST server to set non-default settings.

if [[ "$PROFILE" == "NIA" ]]; then # these settings are for nia's testing version.
  export CLIENT_ID="ada06a30b4de42bdb05344412d2ea21e";
  export BIND_TO="127.0.0.1:8443";
  export RUST_LOG="debug";
fi