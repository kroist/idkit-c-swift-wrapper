#!/bin/bash

set -e

THISDIR=$(dirname $0)
cd $THISDIR

cargo build -p idkit-swift

swiftc -L ../../target/debug \
  -lidkit_swift \
  -import-objc-header bridging-header.h \
  -framework CoreFoundation -framework SystemConfiguration \
  main.swift ./generated/SwiftBridgeCore.swift ./generated/idkit-swift/idkit-swift.swift
