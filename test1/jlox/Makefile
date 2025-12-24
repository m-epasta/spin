JAVAC = javac
JAVA  = java
SRC   = $(shell find lox -name "*.java")
OUT   = out

.PHONY: all run clean

all:
	mkdir -p $(OUT)
	$(JAVAC) -d $(OUT) $(lox)

run: all
	$(JAVA) -cp $(OUT) com.craftinginterpreters.jlox.Lox

clean:
	rm -rf $(OUT)
