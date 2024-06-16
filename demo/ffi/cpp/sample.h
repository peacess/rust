
#ifndef SAMPLE_H
#define SAMPLE_H

#include "data.h"

extern "C" {
void f_int(int i);
void f_int_ref(int &i);
void f_int_p(int *p);

void f_char(char c);
void f_char_ref(char &c);
void f_char_p(char *p);

void f_data(Data d);
void f_data_ref(Data &d);
void f_data_p(Data *d);
}

#endif // SAMPLE_H