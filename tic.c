// build with: gcc -Wall tic.c -O3 -o tic && strip tic

#include <stdio.h>
#include <sys/time.h>

int main() {
  struct timeval tp;
  gettimeofday(&tp, NULL);
  long int ms = tp.tv_sec * 1000 + tp.tv_usec / 1000;
  printf("%ld\n", ms);
  return 0;
}
