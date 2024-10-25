# Directory tree
Thank you for taking the time to show us your coding skills. This challenge
allows us to see your ability to solve an everyday problem. There are no
tricks or hidden gotchas.
 
We will evaluate your effort on the following criteria (in order of importance):
- **Does it work?** We should be able to run your code and see the correct output.
- **Is it clean?** Within the time constraints, please give us your best effort at "production code".

Use any language you feel comfortable in. Please include a README with all necessary instructions for getting your code to run.

Please share the submission via a file-sharing service such as Github or Dropbox (not Google Drive), attachments will be blocked.

## of note!
While we guide most people to spend 2-4 hours, please send us whatever you have completed after roughly 48 hours.  A complete solution is an important metric for us, but it's not the only one. 

We do our best to review these challenges anonymously - so please do your best to not include personally identifiable information in your submission (but not at the expense of extra time spent working on this challenge)

We also want to see clean code structure, clear separation of concerns and whatever else you deem important in "production code".  So if you don't finish, send us what you've got -- we want to see it!

## Questions?
Email us at  jfranco@endpoint.com, khidalgo@endpoint.com, AND jross@endpoint.com

## Deliverable
We're expecting you to send your solution as a single page app, command line script or executable.  Some examples:

```bash
$ node directories.js
$ ruby directories.rb
$ python directories.py
$ yarn start
```

If you are doing the challenge with compiled code, please deliver both the source code and an executable or instructions for creating it.
Please share the submission via a file-sharing service such as Google Drive

## The problem

A common method of organizing files on a computer is to store them in hierarchical directories. For instance:

```
photos/
  birthdays/
    joe/
    mary/
  vacations/
  weddings/

```

In this challenge, you will implement commands that allow a user to create, move and delete directories.

A successful solution will take the following input:
```
CREATE fruits
CREATE vegetables
CREATE grains
CREATE fruits/apples
CREATE fruits/apples/fuji
LIST
CREATE grains/squash
MOVE grains/squash vegetables
CREATE foods
MOVE grains foods
MOVE fruits foods
MOVE vegetables foods
LIST
DELETE fruits/apples
DELETE foods/fruits/apples
LIST
```

and produce the following output
```
CREATE fruits
CREATE vegetables
CREATE grains
CREATE fruits/apples
CREATE fruits/apples/fuji
LIST
fruits
  apples
    fuji
grains
vegetables
CREATE grains/squash
MOVE grains/squash vegetables
CREATE foods
MOVE grains foods
MOVE fruits foods
MOVE vegetables foods
LIST
foods
  fruits
    apples
      fuji
  grains
  vegetables
    squash
DELETE fruits/apples
Cannot delete fruits/apples - fruits does not exist
DELETE foods/fruits/apples
LIST
foods
  fruits
  grains
  vegetables
    squash
```

## Helpful hints
    Please solve the challenge on your own and without using any helper libraries as this would not show us the skillset we are interested in.
    Your solution should not actually create folders on the host machine.
    Your solution should take the above input and produce exactly the output shown above.
