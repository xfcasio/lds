CC=clang
CFLAGS=-Wall -Wextra -g

.PHONY: clean run test

main: main.c *.h *.c
	@$(CC) $(CFLAGS) -o main main.c

run: main
	@./main

test: tests.c *.h *.c
	@$(CC) $(CFLAGS) -o tests tests.c
	@./tests
	@rm -f ./tests

clean:
	@rm -f main
