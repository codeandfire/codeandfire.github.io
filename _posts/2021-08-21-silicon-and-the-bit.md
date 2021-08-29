---
layout: post
title: Silicon and the Bit
date: 2021-08-21 16:33 +0530
---

In today's world, where we take computing devices - be it a laptop or a mobile phone - almost for granted, we often forget the fundamental units that drive computing in our everyday lives. These units are a class of materials known as _semiconductors_, the most important and well-known of them being silicon - and the _transistor_, an important electrical device built using semiconductors.

In this post, we go over the basic physics involving semiconductors like silicon, and transistors, in highly simplifed terms. Then, we discuss how the transistor can be used as a 'switch', and how that leads to a convenient representation of a bit. Finally, we briefly look at alternative paradigms of computing, like DNA computing and quantum computing.

### Semiconductors

Elements that make good conductors of electricity essentially have the following property: the electrons in the outermost shells of their atoms are extremely loosely bound to the parent nucleus, so when an external electric field is applied, these "free" electrons can participate in carrying current. 

In a semiconductor, the situation is a little different. It turns out that the electrons in the outermost shells are not as "free", and _by default_ cannot participate in carrying current. However, these semiconductors are solids - they are in the solid state. And in the solid state, the atoms are present very close to each other (unlike in a liquid or a gas), so in their random motion these atoms typically keep "brushing against" each other. As a result of this "brushing around", it is possible that, a _few_ electrons get "kicked" enough to gain energy to temporarily "free" themselves from their parent nuclei, and participate in carrying current.

(An insulator is a material in which "freeing" an electron from its parent nucleus requires so much energy that it is nearly impossible for any amount of "brushing around" to provide it.)

As is obvious from the name, a 'semiconductor' is a worse conductor of electricity than a good conductor - better than an insulator, but still worse than a conductor. Then why would we ever use a semiconductor over a conductor? The reason for this is that the conductivity of a semiconductor can be carefully _tuned_ and tailored according to your requirement - which is not possible in the case of a conductor.

The process by which the conductivity of a semiconductor can be tuned is known as _doping_ (in a good sense). Doping involves carefully "diffusing" another material - known as an _impurity_ - into the semiconductor material, as a consequence of which, some number of the semiconductor atoms are replaced by the atoms of the impurity material.

Take silicon as the semiconductor, for example. One type of doping, known as _n-type_ doping, involves diffusing an impurity like phosphorous into the silicon crystal. Silicon has 4 electrons in its outermost shell, and with these 4 electrons it forms bonds with 4 neighbouring silicon atoms in the silicon crystal. Phosphorous, on the other hand, has 5 electrons in its outermost shell, and when it takes the place of a silicon atom in the silicon crystal, it uses 4 of its 5 electrons to form bonds with the neighbouring 4 silicon atoms. The fifth electron is very loosely bound to the parent phosphorous nucleus, and can be easily "freed" to participate in conduction. So, addition of phosphorous essentially provides more electrons for conduction.

![silicon-phosphorous](/assets/silicon_bit/silicon-phosphorous.jpeg)

Only "free" electrons do not participate in conduction. Electrons can even "jump" around from one silicon atom to another, if there are vacancies - also known as _holes_ - in the second silicon atom, and this motion also helps in carrying current.

In the second type of doping, known as _p-type_ doping, we use an impurity like boron, which has 3 electrons in its outermost shell. When boron takes the place of a silicon atom in the silicon crystal, it uses these 3 electrons to form bonds with 3 out of the 4 neighbouring silicon atoms - it is unable to form a bond with the fourth. This vacancy, or hole, can be easily filled by an electron - an electron that was otherwise "clinging" to a silicon atom and participating in a silicon-silicon bond: which means that this electron in turn leaves behind a hole between two silicon atoms, for some other electron to fill. In other words, the addition of boron provides more holes in silicon atoms for conduction.

![boron-hole](/assets/silicon_bit/boron-hole.jpeg)

In both types of doping, the amount of the impurity added to the semiconductor material can be carefully controlled, in order to get precisely the desired conductivity. This is what makes semiconductors so powerful.

### p-n Junction

Something very interesting happens when a single piece of semiconductor material is doped with p-type impurities on one side, and n-type impurities on the other: we get a p-n junction.

![pn-junction](/assets/silicon_bit/pn-junction.jpeg)

A basic law of nature says that if there is more of something on one side and less of it on the other side, then it must cross over - or _diffuse_ - to the other side. At a p-n junction, there are more holes on the p-side and more electrons on the n-side, so they begin to diffuse across the junction and cross into the n- and p-sides respectively.

But as electrons cross over to the p-side, and holes cross over to the n-side, negative charge begins to accumulate on the p-side and positive charge begins to accumulate on the n-side. The positive charge right next to the junction on the n-side begins to pull the electrons back to n-side, and likewise the negative charge right next to the junction on the p-side begins to "pull" the holes back (of course the holes are not some sort of positively charged particles, but can be considered equivalent to one). These two opposing motions counterbalance each other and somewhere an equilibrium is reached, leading to a p-n junction that looks like this:

![pn-junction-2](/assets/silicon_bit/pn-junction-2.jpg)

The charges on either side of the junction are restricted to a thin region (known as the _depletion region_).

Now, let us put this p-n junction in a circuit, i.e. connect it to a battery. There are two ways in which we can connect the battery:
  1. Connect the positive end of the battery to the p-side and the negative end to the n-side. This arrangement "helps" electrons to cross over to the p-side, and holes to cross over to the n-side, and is known as _forward bias_.
  2. Connect the negative end of the battery to the p-side and the positive end to the n-side. This arrangement pushes the electrons on the p-side back to the n-side, and holes on the n-side back to the p-side, and is known as _reverse bias_.

The following diagram shows the reverse bias circuit on the left, and the forward bias circuit on the right:

![reverse-forward](/assets/silicon_bit/reverse-forward.jpeg)

### Bipolar Junction Transistor

Now, when we take a single piece of semiconductor material, and dope it three ways:
  1. lightly dope the thin region in the middle,
  2. moderately dope a thick region at one end,
  3. and heavily dope a region (of moderate thickness) at the other end,

we get a bipolar junction transistor. The three regions described above are known as the _base_, _collector_ and _emitter_ respectively.

![npn-transistor](/assets/silicon_bit/npn-transistor.jpeg)

If the base is doped with a p-type impurity, while the collector and emitter are doped with n-type, we get an n-p-n transistor. Likewise, if the base is doped with an n-type impurity, while the collector and emitter are doped with p-type, we get a p-n-p transistor. In both cases, we have two p-n junctions on the same semiconductor material, and this is what leads to some interesting properties.

(There aren't any transistors that are p-n-n or n-n-p or any such combination, because these wouldn't have two p-n junctions - they would have only one.)

### Transistor as a Switch

In the following discussion we will use the n-p-n transistor: everything applies equally to the p-n-p transistor.

It turns out that in a certain configuration (circuit), the transistor can be used as a switch. (What do I mean by "switch"? We will see.) Basically, we connect the emitter-base junction in forward bias, and the collector-base junction in reverse bias. Furthermore, the collector is connected to the base via the emitter. What this means is that we have a wire connecting the emitter and base, and another wire connecting the collector and emitter - so, essentially, through the emitter connection, the collector is connected to the base.

![switch-circuit](/assets/silicon_bit/switch-circuit.jpeg)

The wire between the emitter and base is referred to as the _input_ part of the circuit, while the wire between the collector and emitter is the _output_ part of the circuit.

Let us see what happens in an n-p-n transistor in this sort of configuration. Essentially, the forward bias at the emitter-base junction aids diffusion of electrons from the n-type emitter to the p-type base (and also holes from the base to the emitter, but the base is only lightly doped so there aren't many holes to participate in that diffusion). Now, the key point is that the base is very thin: so most of the electrons coming from the emitter into the base end up straying over to the collector side. And then, the reverse bias at the collector-base junction aids in pulling these electrons towards the n-type collector and coming out of the collector as current.

Now, let us think about what would happen if we were to apply a high voltage across the input circuit ("high" as in, of the order of 5-6 V). The higher the voltage, the larger will be the number of electrons drawn from the emitter into the base, and then into the collector, leading to a large current (a few mA) in the output circuit.

In many cases, voltage and current are complementary quantities: if one increases, the other decreases and vice-versa. Consider water falling from a height, as a waterfall: you can liken voltage to the height of the waterfall, and current to the thickness of the waterfall; if you increase the height from which the water is falling, the thickness of the stream of water will decrease and vice-versa. And this is what happens in the output circuit: the large current leads to a low voltage across the output circuit.

When a low voltage is applied across the input circuit ("low" as in, of the order of 0.1-0.2 V), it is not of much help in enabling the electrons from the emitter to cross-over into the base and the collector. Due to this, there is a very small current (almost zero) in the output circuit, and the voltage across the output circuit is large.

You can see what is happening: low input voltage leads to high output voltage, and high input voltage leads to low output voltage. The transistor behaves like a switch.

### Representing a Bit

A bit, or a _binary digit_, is essentially a quantity that takes on one of two values: 1 or 0.

Now, consider the following: let high voltage denote a bit with value 1, and low voltage denote a bit with value 0. Then, you can see that the transistor circuit we just discussed essentially behaves like the boolean operation NOT.

That's interesting, isn't it? NOT is not the only operation: it turns out that bipolar junction transistors can be used to construct all the other boolean logic gates, like OR and AND.

Silicon is not the only semiconductor, and bipolar junction transistors are not the only kind of transistor. Modern-day designs of semiconductor chips, transistors and logic gates are much more sophisticated than the simple model we have discussed here, nevertheless the principle stays the same.

Voltage below a certain threshold and above another one, is indeed one of the primary ways of representing a bit, in a modern CPU. Other ways include:
  - Current below and above certain thresholds (CPU).
  - On and off positions of an electrical switch (RAM).
  - Opposing directions of magnetic polarization (hard disk).
  - Changes in reflected light intensity (optical discs like CDs, DVDs).

### Computing Paradigms

So, you can see that, all it takes to build a computer (loosely speaking) is:
  - a physical representation of two (or more) distinct states,
  - and mechanisms to implement the various logical operations using this physical representation.

In other words, there is no constraint to stick to the semiconductor-transistor-based model of computing. And this is where alternative computing paradigms come in. Take DNA computing, for example. DNA occurs in a double-stranded, ladder-like structure in which each "rung" of the ladder is made up of a single pair of molecules. There are four unique pairs (adenine-thymine and guanine-cytosine, and their reverses thymine-adenine and cytosine-guanine), and they can be used to denote four distinct states. Logical operations such as AND, OR and NOT are performed using chemical reactions!

You can say that the popularity of any chosen physical representation depends on two things:
  - the speed of computing that it offers,
  - and the ease with which it can be constructed, maintained and used.

On these lines, you can realize that the transistor-based model of computing became ubiquitous essentially because of two things. Firstly, computations are carried out at the speed of electricity, i.e. it is very, very fast. And secondly, technological advancements, specifically the inventions of the [integrated circuit](https://en.wikipedia.org/wiki/Integrated_circuit) and the [MOSFET](https://en.wikipedia.org/wiki/MOSFET) enabled the _miniaturization_ of the transistor, so much so that modern-day transistors have a size of the order of micrometers and nanometers, and billions of them can be fit into a single small chip (and of course we have [Moore's law](https://en.wikipedia.org/wiki/Moore%27s_law)).

DNA-based computing, on the other hand, is not as convenient: the chemical reactions have to be carried out in test tubes, using sophisticated procedures and techniques, and that's probably why we don't use DNA in our laptops (yet). But such paradigms are not being proposed to replace silicon chips and transistors: rather, they may be able to augment them. For example, DNA computing might be able to revolutionize data storage, because, as we all know, DNA is a super-efficient form of data storage - it stores the entire information about what essentially makes us human, inside the nucleus of each and every cell in our body!

The other motivation behind alternative computing paradigms is that at certain, special problems, they may be much, much faster than conventional computing. For example, the original [claim-to-fame](https://en.wikipedia.org/wiki/DNA_computing#Applications,_examples,_and_recent_developments) of DNA computing was that it might be able to solve the well-known [Travelling Salesman Problem](https://en.wikipedia.org/wiki/Travelling_salesman_problem) in a matter of seconds, while a conventional CPU would take several years.

### Quantum Computing

These days, when it comes to alternative paradigms for computing, the most popular one is quantum computing.

Quantum computing is a little bit different from just offering yet another physical representation for two or more distinct states. The idea here is that states are no more distinct: more formally, we say that each state is a superposition of a finite number of discrete states, and collapses to one of these discrete states upon measurement. So, you no more have a bit which is either 0 or either 1: rather, you have a _qubit_ which has a certain probability of being 0, or being 1, when measured. In order to fully appreciate this statement, you need to understand quantum mechanics, but that is a topic for another day.

### Further Reading

To understand the physics involved in semiconductors and transistors in-depth, you may refer to the following series of lectures on 'Semiconductors' by Prof. H. C. Verma of IIT Kanpur:

  1. <https://www.youtube.com/watch?v=8luE9L8bj4Y>  
  2. <https://www.youtube.com/watch?v=l0nvIh34eug>  
  3. <https://www.youtube.com/watch?v=hTPIiC70WCg>  
  4. <https://www.youtube.com/watch?v=vnQ4uovIwR8>  
  5. <https://www.youtube.com/watch?v=ptUPen8U5yE>  
  6. <https://www.youtube.com/watch?v=P11paY2vACc>  
  7. <https://www.youtube.com/watch?v=ijNBnXhiy_w>  
  8. <https://www.youtube.com/watch?v=oXY9zUSulkg>
