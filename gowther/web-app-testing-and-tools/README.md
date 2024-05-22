# Web App Testing & Tools

- Course on frontend master: https://frontendmasters.com/courses/web-app-testing/
- Companion repo: https://github.com/wtlin1228/testing-fundamentals
- Guide to testable code: https://github.com/wtlin1228/guide-to-testable-code
- Slides: https://drive.google.com/file/d/1ssO7xyK3NPFGcqJWE2hif5BxSC1exLlH/view?usp=drive_link

# My Takeaways

1. Setup your environment so that writing/running the test is easier than the application.
2. Testing is about what do you need to do to make the code testable, not about how do you actually test.

# Why Test?

Why should we test? Because of "Laziness"

It's the path of least resistance.

### Improved Code Quality

- Early bug detection
- Refactoring confidence (#1)
- clearer code documentation

#1 - Moving things around is scared. Especially if it's a codebase that is not yours. What typically happens is that you come to a project and you're like, I see that there's a problem, I should refactor it. But do I really wanna take the risk of like, I do some refactoring, and as a result, it will make things broken, and I'll have to go and make some bug fix something forever, right? And so a lot of people don't take that risk, which is completely opposite of what you want, because now your solution isn't the proper solution. Your solution solution is some hack that you place in place to fix your problem. But actually make the overall problem worse, right? So refactoring confidence is actually a super, super important thing.

### Increased Productivity

- Faster debugging
- Reduced regression
- Code maintainability

### Team Collaboration

- Shared understanding
- Communication
- Continuous integration
- Team members vs Future self

### Confidence

- Reduced risk of prod issues
- Better user experience
- Peace of mind

## Reduced Cognitive Load

As a human, there's limited number of things we can keep track of. And another thing to point out is that a lot of times in organization, things go bad. And people tend to have this thing of, let's blame somebody or let's blame something, right? And people often come to be like, you didn't do this or x or something like that. But that's not useful. Because at the end of the day, you can't have a million things that you expect the developer to do, that's just not scalable.

At some point, you should just say like, there's only one thing that the developer has to do, and that one thing is automated, which is running the test, right? As long as the test say it's okay, then you haven't forgotten anything. So you can't have a situation where you put the responsibility of remembering to check the boxes or do bunch of things on a developer because first of all, developers leave and come, so you always have a new influx of developers. But also there's a limited number of checklists that you can have a developer before this becomes not scalable.

And so instead, you really wanna offload this work somewhere else, right? And it comes down to the cognitive load of the developer, right? You wanna make sure that the developer doesn't have to think about it. Or to put it differently, the developer has limited capacity to do stuff, and you wanna make sure that you use as much of the capacity as possible for the problem at hand, which is for the business logic or for whatever you're trying to achieve, right? And you're trying to offload all of these secondary things because the more the developer has to keep in their heads, the less is left over to actually do useful work.

### Safety Net / Feedback

- Immediately see if a change is breaking

### Remember less

- Form of documentation
- Assumptions are codified

### Debugging

- Narrow the problem through
- Debug only what matters

### Confidence

- To refactor
- To add feature
- To fix bug

# Why Don't you test?

## Semi-Valid Excuse

- Legacy codebase
- I write UI

## Common Misconception

### Dirties design

People say, well, it dirties the design. And I'm gonna try to convince you actually no, it makes the design better. Because fundamentally what testing requires you to do is to assemble the system in a different way. Like if the codebase you have can only be assembled in one particular way which runs the application, and you come in from a testing point of view, say, well actually, I wanna assemble it in the smaller chunks. That means that actually it's less coupled because it gives you the freedom to assemble it, not the whole app, but a portion of it and run the portions of it independently. So that actually make the testing easier.

Now the reason people say dirties the design is because if you have a legacy codebase, it is so intertwined that trying to get it to a testable state, you might temporarily have to kind of hack things, too, in order to make them work.

### Doesn't catch bugs

People will sometimes argue that the testing doesn't catch bugs, which is kink of a strange argument for me because it's kind of the whole point of it, and especially once you have lived in a codebase that has good tests and you refactor the codebase, you realize just how invaluable it it and you realize like, how did we used to write code before testing?

### It's slower

People argue that it's slower because you have to do more work. So it's true that with tests, you'll do more typing, because you have to write tests, right? But I actually think it is faster, especially once if you start a new project, you write lots of code fast, but that quickly kind of tapers off, and you get into kind of a maintenance mode, right? And it's in the maintenance mode where tests really start to shine because you think you're fixing a bug so you change something and unless you have all the previous tests and all the previous assertions, your fix might very well be breaking three other things. And so that amount of time you save not regressing is actually huge.

### It's boring

People say it is boring. I actually think it is fun once you're doing it the right way.

### Hard to change

People often argue that if you make a lot of tests, the codebase becomes hard to change. And I see where they're coming from. They're basically saying that the tests assert the inner working so tightly that kind of change to inner workings would require you to change the test.

There is a little bit of merit in it, but it all depends on the kind of test you write. If you have tested essentially what I would call a snapshot test, a test that takes a snapshot of the system and basically any change to that snapshot is considered a failure, those tests are not very useful, right? And also become very difficult to change.

But if you do it correctly and you have intentions in your test saying like, hey, when this is the input of the system, then I expect this particular output. And I specifically don't care what happens in the middle, it's a particular kind of test, then in that case, it becomes easy to change the internals because the test isn't asserting about how you doing things, it is only caring about the particular outcome.

So whether of not your system becomes hard to change, it really come down to how strong you are about making assertions about how it dees things, right? If you're gonna micromanage the software into delivering the output that you want, then yes, you might get in a situation where there tests are overly protective of the changes.

But if you do it like, I don't care how you get there, I just want to make sure this is the output, then the test actually become much easier to deal with.

### Too many interfaces

Writing tests somethings does mean that you have to create interfaces. Now in TypeScript and in JavaScript, interfaces are not really much a thing because everything is duck type so it's not a big deal.

In other languages like in Java or more strongly typed languages. Interfaces are more of a thing. But yes, interfaces are a useful way of allowing you to manage your dependencies.

### Testing is for QA

The other one people often talk about is, hey, testing is for QA. Like, this is not my job to do testing as a software developer. Testing is actually very much the job of the developer. We need to adjust our assumptions about whose responsibility these things are, mainly because testing isn't that much about writing tests, it's about designing the codebase in a way so that it is testable.

If you design code in a way that is untestable, there's very little that somebody can do after the fact to add tests. In other words, if you design code in an untestable way, you've essentially created a legacy codebase. That's the reason why legacy code bases are difficult to test, is because, well, they weren't designed with testing in mind.

So there's nothing inherently difficult about testing existing code, as long as the code is written in a testable manner, right?

### I write UI

### Don't Know How

# Types of Tests & Tradeoffs

## Kinds of Tests

- Testing Level
  - Unit Tests
  - Integration Tests (functional)
  - System Tests (e2e)
  - Acceptance Tests
- Functionality
  - Functional: does it do the right thing?
  - Performance: does it do it fast enough?
  - Security: does it do it in a secure way?
  - Usability: does it do it in a way where usability matters?
  - Accessibility
- Automation
  - Automated Tests
  - Manual Tests
- Other
  - Regression: add tests to prevent similar bugs happen again
  - Smoke: only run a small portion of tests since the system is large and running the tests takes much time

## Tests Level Tradeoffs

- Scope
  - Unit Tests: Lowest scope, focusing on individual units like functions or classes.
  - Integration Tests: Broader scope than unit tests, focusing on how units interact within a module or subsystem.
  - System Tests: Even broader scope, covering the entire system as a whole, integrating all components and functionalities.
  - E2E Tests: Highest scope, aiming to mimic real user behavior and interactions with the complete system.
- Speed
  - Unit Tests: Fastest to write and execute due to their limited scope and isolation.
  - Integration Tests: Generally faster than system and E2E tests, but slower than unit tests due to increased complexity.
  - System Tests: Can be slower than integration tests due to the larger scope and potential dependencies on external systems.
  - E2E Tests: Often the slowest due to their comprehensive nature and potential reliance on real user environments.
- Isolation
  - Unit Tests: Highest isolation, focusing on individual units without external dependencies.
  - Integration Tests: Moderate isolation, testing interactions within a module but potentially having dependencies on external modules or mock objects.
  - System Tests: Lower isolation, often involving real or simulated external systems and dependencies.
  - E2E Tests: Lowest isolation, typically running in the actual user environment with real dependencies.
- Other
  - Maintainability: Unit tests are generally easier to maintain due to their simpler scope. As the scope increases, maintaining tests can become more complex.
  - Cost: Unit tests typically have lower development and execution costs due to their faster nature. The cost increases with broader scope and complexity.
  - Confidence: While each type provides confidence in different aspects, E2E tests often offer higher confidence in the overall user experience due to their comprehensive scope.

# Test Examples

## Unit Tests

Dependency injection!

See https://github.com/wtlin1228/testing-fundamentals/blob/lesson-7/src/routes/github/%5Buser%5D/%5Brepo%5D/github-api.spec.ts

## UI Tests

Storybook!

See https://github.com/wtlin1228/testing-fundamentals/blob/lesson-7/src/clustering/cluster.stories.tsx

## E2E Tests

Playwright and page objects!

See https://github.com/wtlin1228/testing-fundamentals/blob/lesson-7/tests/cluster.spec.ts

# Development Model

The most people as software engineers do is they write code and also produce bugs. And then they have some QA and they expect that the QA will catch their bugs. And then you hire some test engineers and say, why don't you automate all this stuff so we don't have to do it all the time. So, for some reason, when you talk to people, they always assume the testing magic happens at this level. They're like, get some magical software, pay company some money or whatever and this problem will go away, right? This is not the correct mental model, you cannot fix the problem at this level.

And fundamentally, the reason we cannot fix the problem is, I have this opinion, philosophy, I don't know what you wanna call it, which is that the person who creates a mess needs to be the person who suffers the consequences, right? If the two people are different, nothing ever is gonna get better, right? So if the developer is the one that's creating a mess making the code untestable and the automation engineer is suffering the consequences, this isn't gonna get better, right? You need to make sure that the software engineer, not only is responsible for writing the code, but also is responsible for writing the test. And then they see the consequences to their actions, and now they have a closed loop and immediately they can learn and say, I shouldn't do this, it hurts when I do that, don't do that, right?

# Structuring Code for Testability

Get more control of your code by decoupling, dependency injection.

# How to make it hard to test code?

Most people say:

- Make things private
- Using final keyword
- Long methods
- ... ??? ...

Real issues:

- Mixing new with logic
- Looking for things
- Work in constructor
- Global state
- Singletons
- Static methods
- Deep inheritance
- Too many conditionals

So when you ask people you're an evil developer, what make code is hard to test? They'll say, ooh, make things private, using final keyword would be cool into saying object.freeze in JavaScript. Have methods that are very complicated. And sure, to a lot of degrees, this makes the situation harder because you cannot monkey patch things, because the object is frozen. Or you cannot get a hold of things because they're hidden somewhere in a special property. Or the method is too long then it becomes really complicated to write a test.

But fundamentally, the reason why code is hard to test is because you can't separate it out. You can't isolate it. And the reason you can't isolate it is because the code which is responsible for constructing your application and the code that represents the logic are intermixed, so you can't take it apart.

And the reason why they're intermixed is that you'll have things that looks for other pieces of code, and you'll see a lot of snow as love demeter or the train wreck. You'll have something called A, and you'll say something like A.B.C.D.E.F and you kinda walk the tree to get a hold of something that you want. This can also be done as you call a function that returns another function, which you call another function on it to return whatever you want. And you're just kinda working it instead of just being given the thing that you need in the beginning.

# Deal with Legacy Codebase

## Stage 1: Scenario Tests

Test the whole app as unit by pretending to be a user.

But it's slow and flaky.

- Green:
  - high confidence happy paths OK
  - high initial coverage; impossible to get the corner cases
- Red = Could be Ok, not sure
  - hard to reproduce failures
  - need debugger to figure out what went wrong

## Stage 2: Functional Tests

Test a sub-system of the app with simulators for external dependencies.

Can simulate conditions not possible in scenario tests.

Developers can run these before submitting.

Much Faster & A lot less flaky.

- Green:
  - high confidence sub-system OK
  - lower app coverage, need more of these
  - tests focus on class-interaction
- Red = pretty confident things are bad
  - easier to reproduce failures

## Stage 3: Unit Tests

Test individual classes in isolation.

Can simulate all error conditions.

Developers can run these after each file modification..

Very Fast & No flakiness.

- Green:
  - high confidence class OK
  - lowest app coverage, need a lot of these
  - sure class is OK, not sure class interaction OK
- Red = pretty confident things are bad
  - easy to reproduce failures
  - no need for debugger to figure out what went wrong

# References

- [In-Memory Databases](https://www.sqlite.org/inmemorydb.html) is one option for mocking the database with speed in concern.
