---
layout: post
title: Scripting for Machine Learning
---

These days, Jupyter notebooks are the tool of choice for most data scientists and machine/deep learning practitioners, as well as for many others in the broader domain of mathematical/scientific computing.
The benefits of the notebook can be summarized as follows:
  1. It is a _REPL_, short for _read-evaluate-print-loop_.
  2. The output of each command or _cell_ is recorded, which is useful for storing intermediate results.
  3. Text (markdown) cells interspersed with code cells allow you to document your work in the REPL.

While notebooks are great for producing illustrative, descriptive, demonstrative documents, they scale poorly with increase in code complexity, leading to rather messy and ill-structured code.[^1]
This article makes a case for a _script_-based workflow that can serve as an alternative to notebooks, in the domain of machine learning, data analysis, and scientific computing.

### REPL

What is a REPL, or a read-evaluate-print-loop?
This definition from Wikipedia should make it clear:
> A read–eval–print loop (REPL), also termed an interactive toplevel or language shell, is a simple interactive computer programming environment that takes single user inputs, executes them, and returns the result to the user; a program written in a REPL environment is executed piecewise ... Common examples include command-line shells and similar environments for programming languages, and the technique is very characteristic of scripting languages.

In other words, if you have ever typed `python3` into the terminal and got an output like this:
```
$ python3
Python 3.10.4 (main, Mar 23 2022, 23:05:40) [GCC 11.2.0] on linux
Type "help", "copyright", "credits" or "license" for more information.
>>> 
```
what you have entered into is essentially a REPL, the Python shell.
Other languages, like Clojure, Julia and Lua, also have REPLs.
On the other hand, compiled languages like C, C++ and Rust typically do not have a REPL.[^2]

Then there is IPython, short for Interactive Python.
You can install it using pip:
```
$ pip install ipython
```
and then, if you type `ipython` into the terminal, you will get an output like this:
```
Python 3.10.4 (main, Mar 23 2022, 23:05:40) [GCC 11.2.0]
Type 'copyright', 'credits' or 'license' for more information
IPython 8.3.0 -- An enhanced Interactive Python. Type '?' for help.

In [1]: 
```
What is IPython?
Another REPL for Python.
Better than the plain Python shell, in that it supports syntax highlighting and tab completion.

If you fiddle with IPython a little bit, you might notice something familiar.
See the prompts before the input and the output `In [ ]` and `Out[ ]`?
```
In [2]: text = 'Hello, world!'

In [3]: text
Out[3]: 'Hello, world!'
```
If you are on a Linux/Mac system, try this to list the contents of the current directory:
```
In [4]: !ls -lh
```
You can also try this so-called _magic_ command:
```
In [5]: %%timeit
   ...: for _ in range(1000000):
   ...:     pass
   ...: 
10.8 ms ± 545 µs per loop (mean ± std. dev. of 7 runs, 100 loops each)
```
If you have used Jupyter notebooks before, all of these should seem very familiar to you.
Indeed, the `In [ ]` and `Out[ ]` prompts in Jupyter notebooks, the ability to interact with the system shell by starting the command with a bang `!`, and the `%` and `%%` magics, are all native to IPython.
To sum it up, IPython provides the core REPL functionality of Jupyter.

A REPL, very simply, serves the purpose of being a programmer's scratchpad, a place where you can try things out until you find a solution that works.
The point here is this: if you use notebooks simply for their REPL functionality -- as a scratchpad -- consider using IPython directly instead!

### Command-line Arguments

_Any_ parameters that your algorithm depends on should be passed to the script as command-line arguments.
Command-line arguments are the bread-and-butter of working with scripts; Python has the [argparse library](https://docs.python.org/3/howto/argparse.html) for working with them, and typically, the first part of your script should define and parse these arguments.

If your script is training a machine learning model, a typical set of command-line arguments could be the following:
```
$ python3 model.py --learning-rate '0.05' --reg-type 'l2' \
		   --reg-penalty '0.1' --epochs '1' --test-split '0.25'
```
Here, we have the learning rate, the regularization type (l1 or l2), the regularization penalty, the number of training epochs, and the fraction of the dataset allocated to the test split.
This is a fairly simple setup.

A different use case is something which is more geared towards data analysis, where you would like to extract certain statistics/patterns from the data.
Consider this example.
You have the Iris dataset, with four input variables, petal length, petal width, sepal length and sepal width, and three output classes, Iris setosa, Iris virginica and Iris versicolor.
You want to print some statistics for each of these four input variables, but under two scenarios: the first is when you consider the entire dataset as a whole, and the second is when you restrict the dataset to only a single class.
In other words, the first scenario corresponds to something like this:
```python
df['petal-length'].mean()
```
while the second scenario corresponds to something like this:
```python
df[df['class'] == 'Iris-setosa']['petal-length'].mean()
```
Basically, in the second scenario, you are applying a _filter_ on the dataset before computing the mean statistic.

In this case, the structure of command-line arguments that you could adopt can be the following.
You keep two arguments: one to decide the input variable over which the statistic has to be calculated, and the other to decide the class over which the dataset must be filtered.
The variable argument must be a _required_ argument, while the default value of the class argument must be `None`.
In argparse terms, this can be written as the following:
```python
parser.add_argument(
    'variable',
    choices=['petal-length', 'petal-width', 'sepal-length', 'sepal-width'],
    help='input variable over which the statistics are calculated')
parser.add_argument(
    '-c', '--class',
    default=None,
    choices=['Iris-setosa', 'Iris-virginica', 'Iris-versicolor'],
    help='filter the dataset to retain only data points of this class')
```
and then, you can simply do something like this:
```python
if args.class is not None:
    df = df[df['class'] == args.class]
print('Mean:', df[args.variable].mean())
```
In other words, if a class has been specified, then we will apply the corresponding filter on the dataset, otherwise the dataset remains unchanged -- the core step of computing the mean remains the same regardless of whether a filter has been applied or not.

Then, the script can be run multiple times, with different values of the variable and class arguments, in order to collect all the statistics that you want.
```
$ python3 get_stats.py 'petal-length'
$ python3 get_stats.py 'petal-length' -c 'Iris-setosa'
$ python3 get_stats.py 'sepal-width' -c 'Iris-virginica'
```

In general, if you have a larger number of input variables, and there are multiple types of filters/transformations that you need to apply, you can follow this structure.
Construct optional command-line arguments that allow you to control all the filters being applied.
In the script, apply all the required filters first, and then carry out the core computation/analysis.
Finally, run the script multiple times with different sets of values for the command-line arguments, in order to collect all the outputs that you want.

This workflow -- in which the parameters are passed as command-line arguments -- mandates a separation between the input, the algorithm, and the output.
The input comes in as command-line arguments, the algorithm resides within the script, and the output is what comes on your terminal window after the script executes.

### Logging

Another very handy feature that can be used in scripts is logging.
Python provides the [logging module](https://docs.python.org/3/howto/logging.html#logging-basic-tutorial) which makes logging extremely simple.

One straightforward use case is when you are training a deep learning model: you can use logging to track the loss per batch/epoch.
In that case, you might do something like this:
```python
import logging
logging.basicConfig(filename='train.log', level=logging.INFO)
...
for e in epochs:
    for b, batch in enumerate(batches):
	...
	loss = ...
        logging.info(f'Epoch {e} batch {b}: Loss = {loss:.6f}')
```

However, you need not always log to a `.log` file: in many cases, simple `print()` statements to log to standard output (stdout) are more than enough.
You should use `print()` statements to track the progress of the script, as well as record any information that may be of interest while running the script.
Consider the following dummy preprocessing script as an example:
```python
import pandas as pd
import pickle

print('Reading data.')
data = pd.read_csv(...)

orig_size = data.shape[0]
print(f'Found {orig_size} samples.')

data.drop_duplicates(inplace=True)
new_size = data.shape[0]
print(f'Removed {orig_size - new_size} duplicate samples.')

filename = 'cleaned_data.pkl'
with open(filename, 'wb') as f:
    pickle.dump(data, f)
print(f'Cleaned data saved to {filename}')
```
When you watch this script running in the terminal, the `print()` statements will give you an idea of the various steps that are being run in the script, and will also tell you some useful information about how many samples were originally present and how many were duplicates.

Note also that these `print()` statements act as natural comments.
In other words, as you go through the code, the `print()` statements give you an idea of what is being run, so you don't have to explain yourself with a comment.

Realize how we are able to use logging to record intermediate outputs which may be of interest to us.
At the beginning of this article, we mentioned that the ability of Jupyter notebooks to record the output of each code cell, is often used to store intermediate results.
The point is this: if you set up logging correctly in your script, you will not miss Jupyter's ability to record intermediate results.
After your script runs -- or even after multiple runs of your script (if you pass different command-line arguments in each run and would like to record results in each case) -- you can just copy-paste the contents of your terminal window into say a `.txt` file, and all your results will be stored.
Later on, you can refer to this file and transfer the results to a more "displayable" medium like Markdown or a spreadsheet.

Sometimes -- probably when you have run the script for quite some number of times and you know exactly what it is doing and what the intermediate results are -- you might want to stop the script producing output that floods your terminal window.
One option that many scripts use for this is to have a _verbose_ flag, i.e. only when you run
```
$ python3 myscript.py --verbose
```
will the script write much to stdout, without the flag it will not.
Another variation that some scripts follow is to define a notion of _verbosity level_, i.e. the number of times you pass the verbose argument to the script, is counted as proportional to how much output you want to see on your terminal window:
```
$ python3 myscript.py -vv
```
Here, the two `v`'s count as verbosity level 2, and you will get more printed to stdout than normal.

IMHO, you should avoid the verbosity level construct unless you have a really good reason to use it, because it means that you will probably have if-conditions like these hanging before each and every `print()` statement in your script:
```python
if args.verbosity == 1:
    print(...)
elif args.verbosity == 2:
    print(...)
...
```
The verbose flag construct is better, because there is just a single if-condition:
```python
if args.verbose:
    print(...)
```
But even that if-condition can get annoying, and on Unix (Linux/Mac) systems there is a workaround.
Don't use either of the verbose flag or verbosity level constructs; if you want to turn the output off, just do:
```
$ python3 myscript.py > /dev/null
```
This is a well-known trick on Unix systems to silence the output of a script.
I am not aware of a corresponding Windows equivalent, though one may exist.

Along with standard output there is also _standard error_ (stderr), where errors are meant to be logged.
In Python, `print()` by default writes to stdout.
There are [many ways](https://stackoverflow.com/questions/5574702/how-to-print-to-stderr-in-python) to write to stderr; one way involves `print()` but by explicitly telling it to write to stderr:
```python
import sys
print("...", file=sys.stderr)
```
(Personally I like this approach amongst the other ways of writing to stderr in Python because it gels in well with your other `print()` statements which write to stdout.)
You can use such `print()` statements in your script to report any errors that are encountered while running it.

Since stdout and stderr are sort of the "same", as in they both write to your terminal window, practically it may make no difference whether your `print()` statement writes to stdout or stderr.
However, if you use the above Unix trick, it will matter: that trick will suppress all output written to stdout, but will allow any output written to stderr.
In other words, if you run that command, while regular output will be silenced, errors will continue to be displayed -- which is actually quite useful.

Logging is very useful: use it.

### Comments

In all domains of software, when code becomes complicated and not obvious to understand, there is a tendency to want to explain what the code is doing.
Comments exist precisely for this purpose.

Code in the domain of data analysis / machine learning / scientific computing requires to be explained, too.
Perhaps there is a greater tendency to want to explain code in this domain, especially because it often has an _algorithmic_ nature and each step requires to be understood.
As such: you can make liberal use of comments in your scripts.

We will look at some examples to convey the point.
Consider this snippet first:
```python
# model definition
model = Pipeline([
    ('bow', CountVectorizer(min_df=5)),
    ('tfidf', TfidfTransformer()),
    ('classifier', LogisticRegression(
        fit_intercept=True, penalty='l1', C=1.0, solver='liblinear')),
])

# train model and save to file
model.fit(X_train, y_train)
joblib.dump(model, 'model.pkl')
```
This is a standard piece of code: we are training a logistic regression model (using the scikit-learn API), possibly an NLP one considering the two preprocessing steps, and saving it to file.
Technically, you don't need the comments stating "model definition" and "train model and save to file", because both are fairly obvious.
But it helps to convey the steps in the script: the model definition comes first, then the model gets trained, and finally it gets saved to disk.

Consider this snippet next:
```python
# tokenize each text in texts
tokens = texts.apply(
    lambda text: word_tokenize(text.lower()) if pd.notna(text) else [])

# find the vocabulary of each text
vocab = tokens.apply(set)

# combine the vocabularies of all texts to get the complete vocabulary
vocab = set().union(*vocab.tolist())
```
This snippet is not so standard as the previous one.
Basically, we have a collection of texts, stored in `texts` which is a Pandas series, and we want to find the vocabulary of this collection of texts.
Without the comments, you would probably have only little idea about what this code is doing.

Note though, that we have kept the comments brief.
For example, in the first statement, we don't bother to explain what the if-condition is doing.
We assume that once we have conveyed the main objective of that statement, which is to tokenize each text in `texts`, the reader is smart enough to work out why that if-condition is required (it checks if the given text is an NA value, and in that case it assigns the text an empty list of tokens).
Similarly for the second and third statements: we simply state _what_ they do, not _how_ they do it.

However, sometimes, a longer explanation is warranted.
Consider this snippet:
```python
# extract the probability values corresponding to the words in the expected
# output sequence.
# the sequence length is used to ensure that timesteps corresponding to any
# padding tokens are ignored.
# note that probs = [sequence-dimension, timestep-dimension, word-dimension]
probs_output_seq = probs[s, :seq_lengths[s], output_seqs[s][:seq_lengths[s]]]

# recall that perplexity = (p1 * p2 * ... * pn) ** (-1/n)
# so, log perplexity = -1/n * (log p1 + log p2 + ... + log pn)
# we calculate the perplexity value using this log scale because it is more
# accurate.
# (if we use the original formula, the product of many small probability
# values gives the value 0 because of lack of precision.)
# then, the exponentiation of this log perplexity value gives back the
# original perplexity value.
probs_output_seq = torch.log(probs_output_seq)
perplexity = torch.exp(
    -1/seq_lengths[s] * torch.sum(probs_output_seq)).item()
```
You probably require some background on what this snippet is doing.
Basically, we have a language model -- an RNN.
If you recall how an RNN works, it is fed a batch of input sequences, in which each sequence is a list of words occurring at different positions (timesteps) in the text.
Corresponding to each input sequence in the batch, is an expected output sequence, again a list of words at different timesteps.
Given the batch of input sequences, the RNN produces a tensor of probability values.
It assigns a probability to each word in the vocabulary, at each timestep of each sequence in the batch.

An expected output sequence shares the same length as its corresponding input sequence.
But the various input sequences in the batch may have different lengths!
And, it turns out that a batch containing variable length sequences cannot be fed to a deep learning model.
Hence, what happens is that each input and output sequence in the batch is _padded_, using a special padding token, so that all sequences have the same length.

With that, let us look at what this snippet is doing.
`s` is the index of a pair of input/output sequences in the batch.
`seq_lengths[s]` is the shared length of this pair of input/output sequences.
`output_seqs[s]` is the output sequence under consideration.
And `probs` is the tensor full of probability values, produced by the RNN.
The first comment explains how the first statement is extracting, from `probs`, the probability values corresponding to the words in the output sequence `output_seqs[s]`.
The second comment explains how the second and third statements are calculating a quantity known as the _perplexity_, which is defined as the n-th root of the product of these probability values, where n is the length `seq_lengths[s]` (in fact, the comment elaborates on why we cannot directly use this formula and must use an equivalent, alternative formula instead).

That was a long digression into RNNs.
But the point is that we have used fairly lengthy, elaborate comments in this case.
The situation warrants the use of such comments: without them, we would be clueless about the tensor "wrangling" in the first statement and the calculation in the second and third statements.
In situations that merit the use of long comments, we should not be hesitant to write them.

If you use comments appropriately in your scripts, you will not miss the text (Markdown) cell functionality of Jupyter notebooks, and your algorithm will remain well-documented.

### Imperative Style

A script is meant to follow an _imperative_ style of programming.
It basically stands for a sequence of statements/instructions; it is meant to convey a flow of steps.
As such -- you should avoid defining too many classes or functions in your script.

Consider the following example.
Deep learning-based image classification models typically have this structure in which they output a list of un-normalized scores corresponding to the various image classes, and then these un-normalized scores can be converted into normalized probability values using the _softmax_ function.
The policy is to avoid the softmax computation unless the probability values are explicitly required, because the softmax computation is expensive.
It turns out that in many situations, we can do without the probability values:
  - While training, we do not require the gradient through the softmax operation, hence we do not need to perform the softmax and compute the probability values.
  - While predicting the class of a given image as the class to which the highest probability is assigned by the model, we do not require the explicit probability value, since the score is directly proportional to the probability value -- we can simply choose the class to which the highest un-normalized score is assigned.

However, sometimes, we may want to know the probability values, in order to understand exactly how much the model favours a given class, and in this case softmax is required.

Suppose we have these two situations within a single script.
In both situations, we are conducting inference, but in one situation we don't require softmax whereas in the other, softmax is required.
Now, you might feel that you can smartly encapsulate this logic within a function:
```python
def predict(x, return_prob=False):
    with torch.no_grad():
        y = model(x)
    if return_prob:
	y = softmax(y, dim=1)
    return y
```
(This is PyTorch code; the `no_grad()` is because we are conducting inference i.e. we don't want gradients to be computed.)
Basically, unless we explicitly pass the argument `return_prob=True`, softmax is not performed and we get the un-normalized probability scores.

Next, you might realize that there is another set of two situations that exists while carrying out inference.
While calculating metrics like accuracy or F1-score, the model carries out inference with a batch of input images.
However, in other inference scenarios, the input is not a batch but a single image -- for example, when the user chooses an image of their choice and would like to see the model's prediction for that image.
So you might think that it is better to encapsulate this logic as well within the same function:
```python
def predict(x, is_batch=True, return_prob=False):
    if not is_batch:
        x = x.unsqueeze(0)
    with torch.no_grad():
        y = model(x)
    if return_prob:
	y = softmax(y, dim=1)
    return y
```
In other words, you can specify the argument `is_batch=False` if you have passed a single, solitary image as `x`, and then an empty batch dimension will be added around `x` by means of the `unsqueeze()` method, before passing it as input to the model.

In this way, you can continue adding complexity to the `predict()` function, depending on the kinds of inference situations you encounter in your script.
But the problem with this approach is that it hampers the readability of the script.
As you read your script from top to bottom, at some place you will encounter a call to this `predict()` function.
Then you will jump up to the definition of this function, work your way through the if-conditions, understand what this function is returning, and then return to the point where you left off and resume reading.
This is just the case of one function and will probably not matter.
However, as you add more such functions, your script will gradually become a plain sequence of function calls and you will lose the sense of flow.

A better approach is to not have such a `predict()` function at all.
Whenever you do need softmax and the probability values, write the following three lines directly in your script:
```python
...
with torch.no_grad():
    y = model(x)
y = softmax(y, dim=1)
...
```
Similarly, whenever you need to add an empty batch dimension around a single image, write out the following lines directly:
```python
...
x = x.unsqueeze(0)
with torch.no_grad():
    y = model(x)
...
```
This does mean that you will have to repeat two lines (`with torch.no_grad()` and `y = model(x)`) at more than one place in your script.
But it is worth the readability that you get in return.

What if this was not a question of repeating 2 odd lines, but of repeating 10 lines instead?
In that case, you need to think about the organization of your code.
Can you separate out the pre-processing and the post-processing from the core 10-line-long computation, so that it is independent of both of these and can be written out exactly once?

Of course, there are many situations where you must use functions and classes:
  - A function describing a small, isolated unit of work.
    This is the kind of function that is passed to the `apply()` method of the dataframe as an argument, or that represents a work that has to be parallelized.
  - A class describing a model definition.
    This is the typical pattern in PyTorch models, unlike in Keras which has the more script-like style of sequential `model.add()` statements.
    Such a class typically specifies the model definition in the `__init__()` method, and should have only a small number of other methods.

But you get the point: don't use functions and classes that disrupt the flow of the script.

### Testing

### Glueing Scripts

### Packaging Scripts

## Additional Notes

 - Use a maximum line length of 90 characters.
   Tensor "wrangling" and dataframe "wrangling" statements can get lengthy.
   And breaking these statements doesn't improve readability.
 - Don't do this:
   ```python
   if __name__ == '__main__':
       # script here
   ```
   or
   ```python
   def main():
       # script here

   if __name__ == '__main__':
       main()
   ```
   because that will probably mean over a 100+ lines indented by four spaces and effectively reduce the maximum line length available to you by 4.
   Just ensure that your script is not imported as a module.
   If you are defining something in your script that you do need to be imported by another module, you are probably better off defining it in a separate `.py` module, and having your script and the other module importing this separate module.
   Having your script double up as an importable module is not best practice.
 - Don't use type hints in your scripts.
   Type hints in Python are more suited for modular libraries, whereas scripts benefit a lot from duck typing.

### Footnotes

[^1]: There is an excellent talk by Joel Grus on this topic: [I Don't Like Notebooks](https://www.youtube.com/watch?v=7jiPeIFXb6U).
[^2]: This doesn't mean that it is not possible to build a REPL for these languages. For example take a look at [this REPL for Rust](https://github.com/google/evcxr).
