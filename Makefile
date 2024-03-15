# Define the name of the output file
LIB_NAME := libidkit.a

# Define the target architectures
IOS_TARGETS := aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim

# Define the path for the output
OUTPUT_DIR := ./libs

.PHONY: all clean ios

all: ios

ios: $(IOS_TARGETS)

$(IOS_TARGETS):
 @echo "Building for target $@"
 @cargo build --release --target $@
 @mkdir -p $(OUTPUT_DIR)/$@
 @cp target/$@/release/$(LIB_NAME) $(OUTPUT_DIR)/$@/

# Combine the library for different architectures into a single universal binary
lipo:
 @lipo -create \
  -output $(OUTPUT_DIR)/$(LIB_NAME) \
  $(OUTPUT_DIR)/aarch64-apple-ios/$(LIB_NAME) \
  $(OUTPUT_DIR)/aarch64-apple-ios-sim/$(LIB_NAME) \
  $(OUTPUT_DIR)/x86_64-apple-ios/$(LIB_NAME)
 @echo "Universal binary created at $(OUTPUT_DIR)/$(LIB_NAME)"

clean:
 @cargo clean
 @rm -rf $(OUTPUT_DIR)

XCFRAMEWORK_NAME := IDKit.xcframework

create-xcframework:
 @xcodebuild -create-xcframework \
  -library $(OUTPUT_DIR)/aarch64-apple-ios/$(LIB_NAME) -headers ./include/ \
  -library $(OUTPUT_DIR)/aarch64-apple-ios-sim/$(LIB_NAME) -headers ./include/ \
  -output $(XCFRAMEWORK_NAME)
 @echo "XCFramework created at $(XCFRAMEWORK_NAME)"

clean:
 @cargo clean
 @rm -rf $(OUTPUT_DIR)
 