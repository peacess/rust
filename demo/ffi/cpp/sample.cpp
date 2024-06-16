#include "sample.h"
#include "stdio.h"

void print_d(Data &d) { printf("%d, %d\n", d.a, d.d); }

void f_int(int i) { printf("f_int: %d, p: %p\n", i, &i); }

void f_int_ref(int &i) { printf("f_int_ref: %d, p: %p\n", i, &i); }

void f_int_p(int *p) { printf("f_int_p: %d, p: %p\n", *p, p); }

void f_char(char c) { printf("f_char: %d, p: %p\n", c, &c); }

void f_char_ref(char &c) { printf("f_char_ref: %d, p: %p\n", c, &c); }

void f_char_p(char *p) { printf("f_char_p: %d, p: %p\n", *p, p); }

void f_data(Data d) {
  print_d(d);
  printf("f_data: ,p: %p\n", &d);

}

void f_data_ref(Data &d) {
  print_d(d);
  printf("f_data_ref: ,p: %p\n", &d);
}

void f_data_p(Data *d) {
  print_d(*d);
  printf("f_data_p: ,p: %p\n", d);
}
