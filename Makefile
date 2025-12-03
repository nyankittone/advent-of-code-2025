source_dir := src

ifndef archive_name
$(error archive_name unspecified)
endif

ifndef obj_dir_suffix
$(error obj_dir_suffix unspecified)
endif

ifdef obj_dir_prefix
obj_dir := $(obj_dir_prefix)/$(obj_dir_suffix)
else
obj_dir := $(obj_dir_suffix)
endif

sources := $(wildcard $(source_dir)/*.cpp)
objects := $(patsubst $(source_dir)/%.cpp,$(obj_dir)/%.o,$(sources))

compiler := g++
CXXFLAGS := -std=c++23 -Wall -pedantic-errors -Werror=return-type -g -ggdb -Og

.PHONY: all clean archive

all: archive

archive: $(objects)
	ar rcs $(obj_dir)/lib$(archive_name).a $^

$(obj_dir)/%.o: $(source_dir)/%.cpp $(source_dir)/common.hpp | $(obj_dir)
	$(compiler) $(CXXFLAGS) -c $< -o $@

$(obj_dir):
	mkdir -p $@

