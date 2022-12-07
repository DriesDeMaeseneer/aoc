#include <stdio.h>
#include <string.h>
#include <stdlib.h>
/*
 * Advent of code day 7 :-)
 */
// simple struct holding file information.
struct File{
	char name[64];
	unsigned long size;
};
// simple struct holding directory information.
struct Directory{
	char name[64];
	size_t amt_of_files;
	struct File* files[64];
	size_t amt_of_dirs;
	struct Directory* dirs[64];
	struct Directory* parent;
};

struct Directory* new_dir(char name[], struct Directory* parent) {
	struct Directory* to_return = (struct Directory*)malloc(sizeof(struct Directory));
	strcpy(to_return->name, name);
	to_return->parent = parent;
	return to_return;

}
struct File* new_file(char name[], unsigned long size) {
	struct File* to_return = (struct File*)malloc(sizeof(struct File));
	strcpy(to_return->name, name);
	to_return->size = size;
	return to_return;

}

struct Directory* find_dir(struct Directory* current_dir, char name[]){
	for (int i = 0; i < current_dir->amt_of_dirs; i++){
		if (strcmp(current_dir->dirs[i]->name, name) == 0){
			return current_dir->dirs[i];
		}
	}
	return current_dir;
}

void get_dirname(char current_line[255], char* to_string) {
	for (int i = 5; i < 255; i++){
		to_string[i-5] = current_line[i];
	}
}

// If the line in the file starts with $,
// this function will handle it.
struct Directory* handle_command(char current_line[255], struct Directory* current_dir){
	// Filter out ls commands.
	if (strcmp(current_line, "$ ls\n") == 0){
		return current_dir;
	}
	// Override the current_dir variable
	// with the name of the new variable.
	char name[255];
	if (strcmp(current_line, "$ cd ..\n") == 0) {
		return current_dir->parent;
	}
	get_dirname(current_line, name);
	return find_dir(current_dir, name);
}
void get_offset_name(char current_line[255], char* to_string, int offset){
	for (int i = offset; i < 255; i++){
		to_string[i-offset] = current_line[i];
	}
}

void handle_new_dir(char current_line[255], struct Directory* current_dir){
	
	char name[255];
	get_offset_name(current_line, name, 4);
	struct Directory* dir = new_dir(name, current_dir);

	current_dir->dirs[current_dir->amt_of_dirs] = dir;
	current_dir->amt_of_dirs +=1;
}

void handle_new_file(char current_line[255], struct Directory* current_dir){
	unsigned long result = 0;
	unsigned long offset = 0;
	for (int i = 0; i < 255; i++) {
		if (current_line[i] != ' '){
			result *= 10;
			// - 48 due to ascii table.
			result += (unsigned long)current_line[i] - 48;
			offset += 1;
		}
		else break;
	}
	char name[255];
	get_offset_name(current_line, name, offset);
	struct File* file = new_file(name, result);
	current_dir->files[current_dir->amt_of_files] = file;
	current_dir->amt_of_files+=1;


}
struct DirSize{
	char name[64];
	unsigned long size;
};
struct SizeCollector{
	unsigned long diramt;
	struct DirSize sizelist[255];
};

unsigned long dir_size(struct Directory* current_dir, struct SizeCollector* collection) {
	unsigned long total = 0;
	for (int i = 0; i < current_dir->amt_of_dirs; i++){
		total += dir_size(current_dir->dirs[i], collection);
	}
	for (int i = 0; i < current_dir->amt_of_files; i++){
		total += current_dir->files[i]->size;
	}
	struct DirSize dirsize;
	strcpy(dirsize.name, current_dir->name);
	dirsize.size = total;
	collection->sizelist[collection->diramt] = dirsize;
	collection->diramt+=1;
	return total;
}

unsigned long dir_sizes(struct Directory* current_dir, unsigned long limit){
	unsigned long total = 0;
	struct SizeCollector* collector = (struct SizeCollector*)malloc(sizeof(struct SizeCollector));
	collector->diramt=0;
	// collect sizes.
	dir_size(current_dir, collector);
	for (int i = 0; i < collector->diramt; i++) {
		if (collector->sizelist[i].size <= limit){
			total += collector->sizelist[i].size; 
		}
	}
	return total;
}
unsigned long minimum(struct Directory* current_dir){
	unsigned long total = 700000000;
	struct SizeCollector* collector = (struct SizeCollector*)malloc(sizeof(struct SizeCollector));
	collector->diramt=0;
	// collect sizes.
	dir_size(current_dir, collector);
	unsigned long total_used_space = collector->sizelist[collector->diramt - 1].size;
	unsigned long unused_size = 70000000 - total_used_space;
	for (int i = 0; i < collector->diramt; i++) {
		if (collector->sizelist[i].size >= (30000000 - unused_size)){
			if (total > collector->sizelist[i].size){
				total = collector->sizelist[i].size;
			}
		}
	}
	return total;
	
}
int main(){
	// file pointer.
	FILE* fp;
	fp = fopen("day_7.input", "r");

	// error handeling.
	if (fp == NULL){
		perror("Error opening the file");
		return 1;
	}

	char current_line[255];
	struct Directory* root = new_dir("/", NULL);
	struct Directory* current_layer = root;


	// while file is not ended.
	while(!feof(fp)){
		// get line until the newline character.
		fgets(current_line, 150, fp);
		switch (current_line[0]) {
			// handle commands.
			case '$':
				current_layer = handle_command(current_line, current_layer);
				break;
			// handle directory creation.
			case 'd':
				handle_new_dir(current_line, current_layer);
				break;
			// handles dumb error.
			case '\n':
				break;
			// handle file creation.
			default:
				handle_new_file(current_line, current_layer);
				break;
		}
	}

	printf("total size of dirs with limit 100_000 = %lu\n", dir_sizes(root, 100000));
	printf("minimum size = %lu\n", minimum(root));

	fclose(fp);
	return 0;
}
