# uk-ilr-citizenship-checker

Fully written and deployed on Netlify by help from ChatGPT

https://uk-ilr-citizenship.netlify.app/

## ⚠️ Disclaimer

**This tool is for informational purposes only and should NOT be relied upon as professional legal or immigration advice.** UK immigration rules and eligibility requirements change frequently and are subject to updates by the UK Home Office.

**Always verify your eligibility directly with the official UK Home Office website before making any decisions or submitting applications.** This tool may not reflect the most current rules and regulations.

## Running Locally

```bash
python3 -m http.server 8080
```

## Running Tests

```bash
node test.js
```

### Official docs
https://www.gov.uk/apply-citizenship-indefinite-leave-to-remain					
https://www.gov.uk/indefinite-leave-to-remain-tier-2-t2-skilled-worker-visa/time-uk					
https://www.gov.uk/government/publications/indefinite-leave-to-remain-calculating-continuous-period-in-uk					

### Rules inferred from official docs:
 
For ILR (Skilled Worker visa):
  - Must live in UK for 5 years
  - Cannot spend more than 180 days outside UK in any 12-month period (rolling window)

For Citizenship (after ILR):
  - Must live in UK for exactly 5 years before application
  - Cannot spend more than 450 days outside UK during those 5 years
  - Cannot spend more than 90 days outside UK in the last 12 months




## Future Plans

might eventually use this as an opportunity to explore WASM given I already know rust.

WOW! I am impressed ChatGPT

If you benefited from it and feel like donating, donate to the [drona](https://drona-academy.netlify.app/) most of the fund goes towards educating other women like me. This way, you will get more meaningful applications :muscle:
