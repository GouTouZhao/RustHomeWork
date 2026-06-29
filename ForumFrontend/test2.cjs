const puppeteer = require('puppeteer');
const fs = require('fs');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:5173');
  
  await page.waitForSelector('.nav-toggle');
  await page.click('.nav-toggle');
  await page.waitForSelector('.theme-toggle');
  
  await page.click('.theme-toggle');
  await new Promise(r => setTimeout(r, 1000));
  
  const styles = await page.evaluate(() => {
    const el = document.querySelector('.app-container');
    const comp = window.getComputedStyle(el);
    return {
      display: comp.display,
      minHeight: comp.minHeight,
      height: comp.height,
      visibility: comp.visibility,
      position: comp.position,
      cssText: el.style.cssText
    };
  });
  console.log('Computed styles of .app-container:', styles);
  
  await browser.close();
})();
