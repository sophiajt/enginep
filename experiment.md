# Engine P

Engine-P is an experimental engine for nushell that uses optional parallelism instead of asynchrony.

By default, the pipeline for nushell is now synchronous, built on an abstraction similar to iterators*. 

Notes:

* Actions can't be removed: plugins need them to participate in updating state
  * Unless we give plugins a way to interact with the engine more directly
  * Can we leave actions only for plugins?

Assumptions:

* Something earlier in the pipeline can not change the variables that are visible nor their value.

* Variables get their values from blocks.
  * `let x = 4` should equivalent to `do {|x| } 4` or better yet `{|x| } 4`

* Question - can these just be normal iterators?