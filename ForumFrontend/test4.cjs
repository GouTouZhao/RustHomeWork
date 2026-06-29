const puppeteer = require('puppeteer');

(async () => {
  const browser = await puppeteer.launch();
  const page = await browser.newPage();
  await page.goto('http://localhost:5173');
  
  await page.waitForSelector('.nav-toggle');
  await page.click('.nav-toggle');
  await page.waitForSelector('.theme-toggle');
  await page.click('.theme-toggle');
  await new Promise(r => setTimeout(r, 1000));
  
  const ruleInfo = await page.evaluate(() => {
    const el = document.querySelector('.app-container');
    let info = '';
    for (let sheet of document.styleSheets) {
      try {
        for (let rule of sheet.cssRules) {
          if (rule.selectorText && el.matches(rule.selectorText) && rule.style.display === 'none') {
            info = 'OwnerNode: ' + sheet.ownerNode.outerHTML + '\nRule: ' + rule.cssText;
          }
        }
      } catch(e) {}
    }
    return info;
  });
  console.log('Rule info:', ruleInfo);
  
  await browser.close();
})();
