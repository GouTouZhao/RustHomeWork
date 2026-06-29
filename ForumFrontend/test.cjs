const puppeteer = require('puppeteer');
const fs = require('fs');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:5173');
  
  await page.waitForSelector('.nav-toggle');
  await page.click('.nav-toggle');
  await page.waitForSelector('.theme-toggle');
  
  console.log('Before toggle:');
  console.log('app-container height:', await page.$eval('.app-container', el => el.getBoundingClientRect().height));
  
  await page.click('.theme-toggle');
  await new Promise(r => setTimeout(r, 1000));
  
  console.log('After toggle:');
  console.log('app-container height:', await page.$eval('.app-container', el => el.getBoundingClientRect().height));
  console.log('body height:', await page.$eval('body', el => el.getBoundingClientRect().height));
  
  const bodyHtml = await page.evaluate(() => document.body.innerHTML);
  fs.writeFileSync('after_toggle.html', bodyHtml);
  
  await browser.close();
})();
