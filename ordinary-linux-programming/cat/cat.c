#include <stdio.h>
#include <stdlib.h>
#include <unistd.h>
#include <sys/types.h>
#include <sys/stat.h>
#include <fcntl.h>

static void do_cat(FILE *f);
static void die(const char *s);

int main(int argc, char *argv[]){
  int i;

  if (argc == 1) {
    do_cat(stdin);
    exit(0);
  }

  FILE *f;
  for (i = 1; i < argc; i++){
    f = fopen(argv[i], "r");
    if (!f) die(argv[i]);
    do_cat(f);
  }
  fclose(f);
  exit(0);
}

#define BUFFER_SIZE 2048

static void do_cat(FILE *f) {
  int n;

  while((n = fgetc(f)) != EOF) {
    if (fputc(n, stdout) < 0) die("fputc");
  }
}

static void die(const char *s) {
  perror(s);
  exit(1);
}
