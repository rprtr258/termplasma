run:
	clang -O3 -mllvm -force-vector-width=32 main.c -lm
	./a.out
