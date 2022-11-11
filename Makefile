SRC_DIR := doc
SRC_FILES := $(wildcard $(SRC_DIR)/*.puml)
OUT_FILES := $(patsubst %.puml,%.svg,$(SRC_FILES))

all: doc
doc: $(OUT_FILES)

# Create doc/xyz.svg from doc/xyz.puml
$(SRC_DIR)/%.svg: $(SRC_DIR)/%.puml
	plantuml -tsvg -o . $<
