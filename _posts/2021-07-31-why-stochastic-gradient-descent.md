---
layout: post
title: Why Stochastic Gradient Descent
date: 2021-07-31 12:29 +0530
---

{% include math.html %}

These days, the algorithm of choice for training deep neural networks and other complex models is gradient descent, and in particular _stochastic_ gradient descent. This 'stochastic' variant involves updating the parameters of the model after receiving each individual sample, as opposed to updating them after iterating through all the samples in the training set - the latter variant is typically known as _batch_ gradient descent.

This post attempts to explain _why_ stochastic gradient descent performs so much better than batch gradient descent: in particular, we discuss why stochastic gradient descent is typically
  - much, much faster,
  - and produces better weights,

than batch gradient descent.

Before we begin, let us recap a bit about gradient descent as well as its batch variant.


### Gradient Descent

In order to minimize a function $f$ which depends on a parameter $\theta$, gradient descent iteratively subtracts a multiple of the gradient of $f$ with respect to $\theta$, from $\theta$. In other words

$$ \theta \leftarrow \theta - \eta \nabla f(\theta) $$

The intuition behind this rule is that the gradient $\nabla f(\theta)$ is the multi-dimensional analogue of two-dimensional _slope_, and that going down the slope of the function leads us towards the minimum of the function. The learning rate $\eta$ controls how far the algorithm goes down the gradient direction.


### The Batch Rule

In machine learning, we have a model $h$ that consists of parameters $\Theta$, and takes an input $x$ and predicts an output. The "correctness" of the predicted output $h(x)$ is evaluated by means of a _loss function_ that compares it with the true output $y$. For example, for regression, we typically use the squared-error loss:

$$ l(\Theta) = \big( h(x) - y \big)^2 $$

Obviously, the objective is to minimize this loss function. But not the loss for a single sample $(x, y)$; rather, we want to minimize the loss over the entire training set of $M$ samples $(x_i, y_i)$. This total loss is

$$ L(\Theta) = \sum_{i=1}^M l_i(\Theta) $$

where $l_i(\Theta)$ is the loss for the sample $(x_i, y_i)$. This is the sum-squared-error loss, the total loss over the entire training set. We can scale this loss by a factor of $\frac{1}{M}$ to obtain the mean-squared-error, the loss on average for each sample in the training set:

$$ L(\Theta) = \frac{1}{M} \sum_{i=1}^M l_i(\Theta) $$

And then, a single _epoch_ of gradient descent involves iterating over each sample $(x_i, y_i)$ in the training set, computing $h(x_i)$ and the loss $l_i(\Theta)$, and at the end performing the parameter update

$$ \Theta \leftarrow \Theta - \eta \nabla L(\Theta) $$

where $\nabla L(\Theta)$ is just the mean of the $M$ gradients $\nabla l_i(\Theta)$. This is the well-known _batch_ gradient descent rule, in which we iterate over the entire "batch" of samples before performing a parameter update.


### Stochastic Approximation

In order to understand the _stochastic_ approximation, let us digress a little.

Imagine you have a large number of people (say a thousand) attending an event, and you want to find the average age of people at that event. How would you go about this? If you wanted to be extremely accurate, you would ask each of the thousand persons their age and then calculate the mean. However, if you just wanted to get a rough idea of the kind of age that these people have, you would probably do something like this: you would pick up a bunch of people - say 10 or 15 - at random, and ask only these persons their age.

(Think about it. This is a very intuitive strategy that we all use.)

It turns out that batch gradient descent and stochastic gradient descent work in a similar fashion as this example. While batch gradient descent chooses to minimize the average loss per sample by calculating the mean gradient of the loss over $M$ samples

$$ \frac{1}{M} \sum_{i=1}^M \nabla l_i(\Theta) $$

stochastic gradient descent _approximates_ this mean gradient by the gradient of the loss $l(\Theta)$ of a _single_, randomly selected sample. 

(We have chosen just a single "person" here, instead of 10-15, but then note that stochastic gradient descent is applied iteratively, so by the end of training we would have selected a very many random samples.)

You save a lot on time by asking 10-15 persons their age instead of a thousand. The same holds for stochastic gradient descent: in the case of batch gradient descent, you perform the first parameter update at the end of one epoch of training, but in stochastic gradient descent, you have probably performed $M$ updates by then. This is what makes stochastic gradient descent _fast_.

Now, realize that our strategy of selecting only a subset of people at an event, in order to estimate the average age of all the attendees, would not work if:
  - There aren't a large number of people at the event. If there were only, say, 50 people at the event, selecting just 10-15 persons would give you an incomplete picture. It would be better to ask everybody their age.
  - You did not select people at random. If you chose 10-15 persons, say, who were sitting next to you at the event, it's entirely possible that people far off from you belong to a different age group altogether.

You can extend this analogy to stochastic gradient descent. Stochastic gradient descent works well only if
  - the training set is sufficiently large,
  - and each sample $(x, y)$ passed to it is randomly chosen, or equivalently, the training set is randomly shuffled before iteratively applying stochastic gradient descent on its samples. 

On a small training set, batch gradient descent would probably converge faster, because each update would be more accurate than its stochastic counterpart, effectively reducing the total training time.


### A Better Alternative

So, stochastic gradient descent is an orders-of-magnitude faster alternative to batch gradient descent, on large datasets, but it is still an approximation of the same algorithm.

If you perform batch gradient descent with a suitable learning rate, you will see a smooth decrease in the mean-squared error $L(\Theta)$ with each update. However, if you run stochastic gradient descent with the same learning rate, you will see a downward trend of course - but accompanied with heavy fluctuations at each update. Clearly, this is because the true minimizer of the mean-squared error $L(\Theta)$ is the batch rule: the stochastic rule approximates the direction of update, and is not guaranteed to reduce $L(\Theta)$ in the same way as the batch rule.

This means that if we had the computational resources, and the time, to perform batch gradient descent, instead of stochastic gradient descent - we should, right?

Not really.

Many times, it turns out that models trained using stochastic gradient descent perform much better than their batch gradient descent counterparts, both on training and test sets. The reason for this is most likely that the fluctuations in the stochastic gradient descent updates are not such a bad thing: probably
  - they enable the algorithm to "escape" local minima and jump to new, potentially better minima,
  - the noisy nature of the updates acts as some sort of inherent "regularization" and prevents overfitting.

The first point explains their improved performance on training sets while the second explains their performance on test sets.

In summary, this quote from Bengio (2012)[^1] sums up why stochastic gradient descent is almost always a better alternative than batch gradient descent, on large datasets:
> Keep in mind that even the true gradient direction (averaging over the whole training set) is only the steepest descent direction locally but may not point in the right direction when considering larger steps. In particular, because the training criterion is not quadratic in the parameters, as one moves in parameter space the optimal descent direction keeps changing. Because the gradient direction is not quite the right direction of descent, there is no point in spending a lot of computation to estimate it precisely for gradient descent. Instead, doing more updates more frequently helps to explore more and faster, especially with large learning rates.


### In Code

One thing to note is that the difference between batch gradient descent and stochastic gradient descent is really a matter of the indentation of a single line of code. While the following is pseudocode for batch gradient descent

```python
grad = 0
for x, y in training_set:
    grad += ...
theta -= eta * grad
```

if you indent the last line, you get stochastic gradient descent!

```python
for x, y in training_set:
    grad = ...
    theta -= eta * grad
```


### Convergence

The fluctuations in the stochastic gradient descent updates have their share of advantages, as we have discussed previously, but also make it difficult for the algorithm to converge to a minimum.

One solution is to use _annealing_, i.e. gradually decrease the learning rate over time. This reduces the magnitude of the fluctuations, and helps the algorithm to settle at a minimum.[^2]

Another solution is to combine the advantages of both batch and stochastic gradient descent by using _mini-batch_ gradient descent. Here we decide a batch size $B$, divide the training set into batches of this size, and perform updates of the form

$$ \Theta \leftarrow \Theta - \frac{1}{B} \sum_{i=kB + 1}^{(k + 1)B} \nabla l_i(\Theta) $$

where $k$ is the number of the batch, ranging from

$$ 1, 2, \dots, \left\lceil \frac{M}{B} \right\rceil $$

If we set $B = 1$, then we are back to stochastic gradient descent, and if $B = M$, then we have batch gradient descent. Higher $B$ will lead to more stable convergence, whereas lower $B$ leads to faster training: you have to navigate this trade-off by setting an optimal value of $B$.


### Notes

Stochastic gradient descent previously went by the name of _online_ training. This refers to the scenario in which the training set is (almost) infinite in size, and the model has to learn in an "online" fashion, i.e. update its parameters with each training sample that arrives.

Note that the version of batch gradient descent that we have focused on here involves minimizing the mean-squared error, and not the sum-squared error. However, everything is still applicable to the version that minimizes the sum-squared error, since the difference between both versions is just a matter of a scaling factor, i.e. the version that minimizes the sum-squared error would use a learning rate that is smaller by a factor of $M$.


### Further Reading

Bengio (2012)[^1] is a practical guide for training deep neural networks with gradient descent. Ruder (2016)[^2] discusses the various variants of gradient descent.

[^1]: Bengio, Yoshua. "Practical recommendations for gradient-based training of deep architectures." Neural networks: Tricks of the trade. Springer, Berlin, Heidelberg, 2012. 437-478.
[^2]: Ruder, Sebastian. "An overview of gradient descent optimization algorithms." arXiv preprint arXiv:1609.04747 (2016).
