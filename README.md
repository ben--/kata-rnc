# Roman Numeral Calculator Kata [![Build Status](https://travis-ci.org/ben--/kata-rnc.svg?branch=master)](https://travis-ci.org/ben--/kata-rnc)

An implementation of Pillar's Roman Numeral Kata with some extra twists to make it interesting.

## Usage

The kata may be evaluated per the unmodified rules by running `make` on a well-configured Ubuntu system. This will build and test the C version of the solution on an Ubuntu 14.04 system with build-essentials installed.

Developers can automatically run unit tests by issuing a `do/auto test` command.

Current CI status can be found on the [Travis](https://travis-ci.org/ben--/kata-rnc.svg) build. Full CI reporting requires a [Jenkins CI](#local-ci) setup.

## Extra Twists

An implementation of Pillar's Roman Numeral Calculator. Adds the following constraints to make it more interesting:

* When in Rome, there are no such things as decimals or integers, so the calculation will be performed with the strings. No numeric digits will be permitted in the source code.
* Despite their lack of understanding of integers, the Romans (who are attempting to leverage a UNIX system to revolutionize bookkeeping) do understand that UNIX uses the mysterious symbols '0' and '1' to represent success or failure and have applied their "When in Rome" rule, unknowingly creating an exception to the previous rule.
* Due to a mistranslation, the Romans have confused the foreign "Docker" with the well-known "Aqueduct" and initially demanded that the build be possible to run without any other host tools intalled. A well-feared gladiator-turned-programmer convinced them that the intimidatingly named "bash" would also be acceptable.
* On long chariot rides, the Romans are frequently without internet access and must be able to re-run builds after an initial environment download is performed.
* In order to maximize learning, and prepare for the future of computing apparati the Kata will be performed simultaneously in both C and Rust.
* Acceptance Tests and CI are essential to any project and must be provided.

## Original Kata

### Roman Numeral Rules
* Roman numerals consist of the following letters: I, V, X, L, C, D, and M which mean one, five, ten, fifty, hundred, five hundred and one thousand respectively.
* An example would be "XIV" + "LX" = "LXXIV"
* Numerals can be concatenated to form a larger numeral ("XX" + "II" = "XXII").
* If a lesser numeral is put before a bigger it means subtraction of the lesser from the bigger ("IV" means four, "CM" means ninehundred).
* If the numeral is I, X or C you can't have more than three ("II" + "II" = "IV" not “IIII”).
* If the numeral is V, L or D you can't have more than one ("D" + "D" = "M" not “DD”)
* The maximum roman numeral is 3999 (MMMCMXCIX)

### Stories

#### User Story: Addition
As a Roman bookkeeper, I want to be able to add two numbers together. So that I can do my work faster with fewer mathematical errors.

#### User Story: Subtraction
As a Roman bookkeeper, I want to be able to subtract one number from another. So that I can do my work faster and with fewer mathematical errors.

## Jenkins CI

Because we can never depend on web services lasting long-term, a Jenkins CI environment is also provided. To bootstrap this, the following instructions should be followed:

* Install Docker on a Linux machine or Docker for Mac on an OS X system
* Run `do/ci` to install and start Jenkins
* Log into the local Jenkins instance on http://localhost:8080
* Create a user account and disable account creation for security purposes
* Run the bootstrap job to generate the actual build jobs
