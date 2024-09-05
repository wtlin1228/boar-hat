// https://vercel.com/blog/demystifying-inp-new-tools-and-actionable-insights
// https://github.com/vercel-labs/await-interaction-response

function interactionResponse(): Promise<unknown> {
  return new Promise((resolve) => {
    setTimeout(resolve, 100); // Fallback for the case where the animation frame never fires.
    requestAnimationFrame(() => {
      setTimeout(resolve, 0);
    });
  });
}

function acknowledgeUserInteraction() {
  // let user know the interaction is being processed, like changing the class name
  // so, this function should be light-weight, ensure no heavy computation here
}

function actuallyChangeThePage() {
  // heavy works like re-layout the whole page should go here
}

async function eventHandler() {
  acknowledgeUserInteraction();
  await interactionResponse(); // put the code below into task queue, so browser can render the acknowledgement
  actuallyChangeThePage();
}
