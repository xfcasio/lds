CC=clang
CFLAGS=-Wall -Wextra -g -lm #-fsanitize=address

.PHONY: clean run test

main: src/main.c src/*.h src/*.c
	@$(CC) $(CFLAGS) -o main src/main.c

run: main
	@./main

test: src/tests.c src/*.h src/*.c
	@$(CC) $(CFLAGS) -o tests src/tests.c
	@./tests
	@rm -f ./tests

clean:
	@rm -f main
