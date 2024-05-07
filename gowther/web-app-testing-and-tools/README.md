# Web App Testing & Tools

- Course on frontend master: https://frontendmasters.com/courses/web-app-testing/
- Companion repo: https://github.com/wtlin1228/testing-fundamentals
- Guide to testable code: https://github.com/wtlin1228/guide-to-testable-code

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
