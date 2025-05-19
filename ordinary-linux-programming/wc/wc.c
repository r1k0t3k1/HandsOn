#include <stdio.h>
#include <unistd.h>
#include <stdlib.h>
#include <fcntl.h>
#include <string.h>

static int do_wc(FILE *f);

int main(int argc, char *argv[]){
  FILE *f;

  if (argc == 1) exit(1);
 
  int lc = 0;
  
  for(int i = 1; i < argc; i++) {
    f = fopen(argv[i], "r"); 
    lc = do_wc(f);
    printf("%s -> line:%d",argv[i],lc);
  }

}

int do_wc(FILE *f) {
  int c;
  int line_count = 0;
  while((c = fgetc(f)) != EOF) {
    if (c == 10) line_count++;
  } 
  return line_count;
}
