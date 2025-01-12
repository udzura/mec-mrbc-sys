default: lib_a

*.o: mrbc-src/*.c
	$(CC) -DMRB_NO_PRESYM -I ./include $(CFLAGS) -fPIC -c mrbc-src/*.c $(LDFLAGS)

lib_a: *.o
	$(AR) rcs libmruby_mrbc.a *.o

clean:
	rm -f *.o libmruby_mrbc.a