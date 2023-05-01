# Holochain Facets
Faceted classifications as a plugin DNA for any data source with unique identifiers.

Here is the generic facet model:

![facets-general](https://user-images.githubusercontent.com/3776081/235473818-5a359eb8-1a82-4179-866f-5b9e2d0d51ff.png)

The FacetGroup is used for applications that has more than one set of facets for different purposes.  If you only have one set of facets, you don't need that.  If you use it, you will need to "hard-code" the name, to use the right set of facets in the right places in your application.  The Facets and FacetValues can then be user defined.

In addition to this base, you will need a way for users to specify which FacetValues apply to the objects they are categorizing.  In hREA, that model looks like this:

![facets-vf](https://user-images.githubusercontent.com/3776081/235474604-298d8fe2-3bd0-432d-907b-9da2ffc09803.png)

