# Dependence probability

The *dependence probability* (often referred to in code as `depprob`) between two columns, x and y, is the *probability that there is a path of statistical dependence between x and y*. The technology underlying the Lace platform clusters columns into views. Each state has an individual clustering of columns. The dependence probability is the proportion of states in which x and y are in the same view,

\\[
D(X; Y) = \frac{1}{|S|} \sum_{s \in S} [z^{(s)}_x = z^{(s)}_y]
\\]

where S is the set of states and z is the assignment of columns to views.

{{#include html/animals-depprob.html}}

**Above.** A dependence probability clustermap. Each cell depresents the probability of dependence between two columns. Zero is white and black is one. The dendrogram, generated by seaborn, clusters mutual dependent columns.

It is important to note that dependence probability is meant to tell you whether a dependence exists; it does not necessarily provide information about the *strength* of dependencies. Dependence probability could potentially be high between independent columns if they are linked by dependent columns. For example, in the three-variable model

<center>
<div class="mermaid">
graph TD;
    X-->Z;
    Y-->Z;
</div>
</center>

 all three columns will be in the same view since Z is dependent on both X and Y, so there will be a high dependence probability between X and Y even though they are statistically dependent, but they are dependent given Z.

Dependence probability is the go-to for structure modeling because it is fast to compute and well-behaved for all data. If you need more information about the strength of dependencies, use mutual information.


