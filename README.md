# ride_for_linux

For blind people, indentation in code is usually something that's just getting in way, and can even break the code if messed up, in languages like Python.

Even though indentation, as an idea, has been invented to actually make coding easier, not more difficult.

Ride is a concept, for now in a form of a code editor, that not just removes the struggless blind coders experience with indentation, but by taking a completely different approach to coding, makes them able to use the same benefits indentation was designed to provide for sighted people.

## How does it work

Ride is built on top of three fundamental principles:

* Tree coding
* Behavior predictability
* Vertical selection

Combined together, these three principles form a powerful system for working with code.

They're universal, and can be (without any modification) used in any hierarchycal text, including various programming languages (C#, Python, rust, Dart, Kotlin, JavaScript), as well as markup languages (HTML, XML, YAML, JSON).

### Tree coding

Let's use the following code snippet as an example:

```
#!/usr/bin/python3

class Foo:

    def __init__(self):
        # This is the constructor of the Foo class
        # We would like it to be short, but in practice, constructors can get quite lengthy
        # But let thisone be just 3 lines long.

    def introduce(self):
        print("I'm the introduction method of the Foo class.")
        print("My purpose is to introduce the specific object instantiated from it.")

    def greet(self):
        print("Hello, my dear user.")
        print("I'm an foo object, and right now, I'm greeting you")

class Bar:

    def __init__(self, foo):
        # another constructor, this time with a parameter.

        self._foo=foo

    def describe_foo(self):
        print("I have one foo object. Let them introduce themselves and greet you.")
        self._foo.introduce()
        self._foo.greet()
```

If you opened this in a text editor, you would be presented with 28 lines of text.\
However, not all of those lines are of actual importance to you. When you open a Python file like this, first of all, you need to know what structures (like classes) does the module consist of, but you're likely not yet interested in the structures themselves.\
When you take the general picture and decide to inspect a specific class, or you have been going after it from the very start, again, not all of the lines of the class (there can be many of them) are likely important for you.\
At the moment, you're not interested in what is each of the contained methods doing, what other structures does it consist of (like for loops, while loops etc.), you're only interested in what's immediately relevant to the class, everything else is just a distraction.

Though, in a standard editor, you don't have much of a choice. Sighted people can visually process indentation by their sight and focus just on what they need, but a blind person needs to inspect every single line to build their mental image.

That's why Ride works differently. To provide the top view and give the user just the relevant information, it always shows only and only the code related to the current level of indentation.

What does that mean in practice?

The snippet above would in Ride look like:

```
#!/Usr/bin/python3

class Foo:

Class Bar:
```

That's it, nothing more, nothing less. If the user decides they're interested in class Foo, because they want to check up or do something there, or simply because it's the first class of the file, they can dive into it (typycal keyboard shortcut Alt+Right arrow), and the view will become:

```
class Foo:

def __init__(self):

def introduce(self):

def greet(self):
```

Again, the only displayed content is the one with the immediate relevance to the Foo class, and it's upto the user to decide, whether they want to work on this level (say define a new method), or dive further.

They could also decide to go back, Alt+Left usually anywhere in the block, returning to the former view.

The technique might resemble code folding. There are indeed some similarities, but also few important differences:

* Ride by default doesn't hide just the deeper code, like the content of methods in the example above, but also the shallower-one. When I'm working with class foo, class Bar is just as irrelevant for me as the content of methods I'm not interested in. You always get to see the introductory line i.e. the line just before the indentation level increases to your current level, this is useful so you could easily access things like class definition, method definition or condition if you're dived in their blocks without the need to dive out and in all the time. But nothing else, pressing Ctrl+Home / End should take you to the beginning and ending of the current block, respectively.
* Since Ride is rather about navigation than hiding, there are no folded/unfolded regions or partial folds. Ride simply follows the rules specified above, all the time. This makes the behavior very consistent and reliable, and also helps to ensure some of the guarantees that will be discussed later.
* In Ride, things not just display like a tree, but also behave like a tree. I.E. in the example above, if you delete a line with a method definition, the whole method will get deleted. If you copy the line, the whole block under it will get copied as well. This behavior is very powerful, since you can easily move around whole structures in the code without the fear of missing anything.

Also note, as declared before, the concept of tree coding is universal. Some IDEs try to provide functionality similar to the one of Ride by introducing features such as method by method navigation, copying methods classes, or blocks of code (in the better cases).

However, these provide you only with limited capabilities, are usually heavily dependent on the specific language (try to navigate Python methods by a C# IDE), and may not even work reliably (we would of course like all classes, methods, functions to be well defined, but for various reasons ranking from simple human errors to compatibility issues, in practice, they don't need to be).

Ride doesn't know anything about any language, and it's not supposed to. It's only source of information is the indentation already present in the code, which is used to build a tree out of the one-dimensional input.

Working with this tree is very powerful, predictable and it works completely independent from any programming or markup language.

### Behavior predictability

Many IDEs contain functions aiming to provide some automation for tasks such as indentation writing or navigation.

However, in my experience, it doesn't take a lot to effectively confuse them, especially in languages where say the indentation can't be deduced, like Python. There is some automation provided, however since I can't predict when is it going to break and I can't detect if it did already, this can lead to very unpleasant surprises.

Ride takes a different approach. Instead of trying to guess what does the user likely want to do, it provides them with an actual way to express it themselves, so there would be no ambiguity.

Specifically in the means of indentation, this includes two rules:

* The user has no access to the actual indentation. Even if they wanted to, they can't change it directly, neither by writing into the spaces (they're not even present) nor deliberately changing the indentation level.
* The only way the indentation can be changed is by few well defined actions. Creating a new block, Shift+Return, creates a new line of a higher indentation than the currently tracked one and the cursor automatically dives in. Creating a new line, creates a new line of the same indentation under the current-one, skipping the embedded block if it's present. Copying/cutting and pasting also adjusts the indentation to the local tree context.

This approach may seem limiting, but in practice, it's very efficient, can prevent a whole palette of errors and speeds up the work considerably by adding the confidence.

Note, Ride sometimes uses the term block to describe all lines of the same indentation level and their sublines bounded by the nearest lines of a lower indentation. It's very convenient, because this is exactly what blocks usually are in various languages. Though, Ride's blocks are defined through indentation, and thus not dependend on a particular syntax (it applies to a C block just as a Python block or a YAML block, as far as the indentation reflects them).

### Vertical selection

Text selection can have various forms in text editors. It's usually possible to select words, sentences, multiple lines, paragraphs, and generally various subsets of the text.

In Ride, due to its nature, a full-fledged selection would bring up multiple questions. For example, what exactly would it mean to select the closing half of one line and opening half of another?\
Should just the ending of the first line be selected, along with its subblock if it exists? And if the second line has a subblock, should it be selected as well, or would just the line beginning qualify? And if yes, where would be the boundary from which the user would be able to select the second line's subblock as well?

Due to these unclearities, Ride introduces vertical selection.

The idea is very simple. The only thing that is possible to select are lines, while selecting a line always selects its subblock as well.

It's not possible to copy a part of a line (like one word) in Ride. However, the vertical selection allows much more powerful operations, moving whole chunks of code around in a very predictable and fearless way, because this limitation makes sure the tree structure stays intact and can be well integrated into other parts of code.

## Installation

### From source

#### Dependencies

* [Rust programming language](https://www.rust-lang.org/tools/install)
* The [Bass audio library,](https://un4seen.com) make sure to use the version appropriate for your architecture, 64-bit in most cases
* GTK 3 development files, on Linux, use your package manager i.e.
    ```sudo apt install libgtk-3-dev```
    On Windows, you can use [gvsbuild.](https://github.com/wingtk/gvsbuild) Make sure to setup the GTK environment variables in your system configuration to make everything available for the Rust compiler.
* On Linux, you need libspeechd-dev and clang to provide speech.
    ```sudo apt install clang libspeechd-dev```

#### Build

```
git clone https://github.com/RastislavKish/ride_for_linux
cd ride_for_linux
git switch development
cd ride
# on Linux, copy the x64/libbass.so library from the downloaded archive to /usr/local/lib, on Windows, copy x64/bass.dll and c/x64/bass.lib to the current working directory.
cargo build --release -q
# Add the sound files
cp -r Sounds target/release
# Launch the program
cargo run --release -q
```

#### Windows version doesn't speak many characters. What's going on?

When you first launch Ride on Windows, you likely notice that lot of elementary characters in character by character navigation, like space, semicolon, hyphen, are not spoken. The reason is that Ride uses your screenreader for speaking aloud individual characters, however, screenreaders only provide functions for reading texts, that obviously ignore the detail you expect in the character mode.

Ride comes up with its own text rendering, specifically, you can define how should a currently focused character be spoken (pressing Ctrl+R), or you can also use phrase -> phrase replacements for line by line navigation (Ctrl+Shift+R).

In the future, presets should be shipped with the program to make this task easier. They're difficult to produce at this moment, since every configuration needs to be specific for a particular language and particular synthesiser.

The Linux version doesn't need any adjustions unless the users want to make them, since the program uses the character API of speech dispatcher.

