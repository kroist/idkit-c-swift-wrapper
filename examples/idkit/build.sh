#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

cargo build -p idkit

swiftc -L ../../target/debug \
  -lidkit \
  -import-objc-header bridging-header.h \
  -framework CoreFoundation -framework SystemConfiguration \
  main.swift ./generated/SwiftBridgeCore.swift ./generated/idkit/idkit.swift
