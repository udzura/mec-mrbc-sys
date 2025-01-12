*.o: *.c
	$(CC) -DMRB_NO_PRESYM -I ../include $(CFLAGS) -fPIC -c *.c $(LDFLAGS)

lib_a: *.o
	$(AR) rcs libmruby_mrbc.a *.o