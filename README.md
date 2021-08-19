# Assembly-Super-Sketchy
Compiler for the Assembly Super Sketchy (ASS) language

Thank you to my friends for helping me come up with a name for the language.

The compiler is written in Rust with a homemade lexer and parser to see what I am capable of post my compiler design class. Hence the name Assembly for an assembly like language and Super Sketchy for me wanting to make my own lexer and parser.
This is a personal project of mine and the goal is to use this language for the code gen of a higher level language I make in the future.
Eventually I want this readme to contain the spec of the language.

Basic Spec:
          Instructions that return values like add move or copy the final register is the destination
          The heap and stack are seperate and grow independently of each other*
          Suppose register aa131 is set value 20 the other aa registers from 0 to 130 will also be initialized to value 0**
          *The plan for right now
          **I think this would be a cool feature
