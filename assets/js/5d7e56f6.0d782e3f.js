(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[443],{3905:function(e,t,n){"use strict";n.d(t,{Zo:function(){return u},kt:function(){return f}});var o=n(7294);function r(e,t,n){return t in e?Object.defineProperty(e,t,{value:n,enumerable:!0,configurable:!0,writable:!0}):e[t]=n,e}function i(e,t){var n=Object.keys(e);if(Object.getOwnPropertySymbols){var o=Object.getOwnPropertySymbols(e);t&&(o=o.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),n.push.apply(n,o)}return n}function a(e){for(var t=1;t<arguments.length;t++){var n=null!=arguments[t]?arguments[t]:{};t%2?i(Object(n),!0).forEach((function(t){r(e,t,n[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(n)):i(Object(n)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(n,t))}))}return e}function s(e,t){if(null==e)return{};var n,o,r=function(e,t){if(null==e)return{};var n,o,r={},i=Object.keys(e);for(o=0;o<i.length;o++)n=i[o],t.indexOf(n)>=0||(r[n]=e[n]);return r}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(o=0;o<i.length;o++)n=i[o],t.indexOf(n)>=0||Object.prototype.propertyIsEnumerable.call(e,n)&&(r[n]=e[n])}return r}var c=o.createContext({}),l=function(e){var t=o.useContext(c),n=t;return e&&(n="function"==typeof e?e(t):a(a({},t),e)),n},u=function(e){var t=l(e.components);return o.createElement(c.Provider,{value:t},e.children)},d={inlineCode:"code",wrapper:function(e){var t=e.children;return o.createElement(o.Fragment,{},t)}},p=o.forwardRef((function(e,t){var n=e.components,r=e.mdxType,i=e.originalType,c=e.parentName,u=s(e,["components","mdxType","originalType","parentName"]),p=l(n),f=r,h=p["".concat(c,".").concat(f)]||p[f]||d[f]||i;return n?o.createElement(h,a(a({ref:t},u),{},{components:n})):o.createElement(h,a({ref:t},u))}));function f(e,t){var n=arguments,r=t&&t.mdxType;if("string"==typeof e||r){var i=n.length,a=new Array(i);a[0]=p;var s={};for(var c in t)hasOwnProperty.call(t,c)&&(s[c]=t[c]);s.originalType=e,s.mdxType="string"==typeof e?e:r,a[1]=s;for(var l=2;l<i;l++)a[l]=n[l];return o.createElement.apply(null,a)}return o.createElement.apply(null,n)}p.displayName="MDXCreateElement"},2524:function(e,t,n){"use strict";n.r(t),n.d(t,{frontMatter:function(){return a},metadata:function(){return s},toc:function(){return c},default:function(){return u}});var o=n(2122),r=n(9756),i=(n(7294),n(3905)),a={title:"governance",description:"Overview of governance in LIDO for Solana",keywords:["governance","multi-sig","lido","solido","solana"]},s={unversionedId:"Guides/Governance/Governance",id:"Guides/Governance/Governance",isDocsHomePage:!1,title:"Governance Overview",description:"Overview of governance in LIDO for Solana",source:"@site/docs/Guides/Governance/Governance.md",sourceDirName:"Guides/Governance",slug:"/Guides/Governance/Governance",permalink:"/docs/Guides/Governance/Governance",version:"current",frontMatter:{title:"governance",description:"Overview of governance in LIDO for Solana",keywords:["governance","multi-sig","lido","solido","solana"]},sidebar:"solidoSidebar",previous:{title:"Solong",permalink:"/docs/Guides/Staking/Wallets/Solong"},next:{title:"Validation Overview",permalink:"/docs/Guides/Validation/Validation"}},c=[{value:"The Lido DAO",id:"the-lido-dao",children:[]},{value:"Governance Rewards",id:"governance-rewards",children:[]}],l={toc:c};function u(e){var t=e.components,n=(0,r.Z)(e,["components"]);return(0,i.kt)("wrapper",(0,o.Z)({},l,n,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h2",{id:"the-lido-dao"},"The Lido DAO"),(0,i.kt)("p",null,"The Lido DAO is a Decentralized Autonomous Organization which governs and enables the development of liquid staking solutions for different blockchains.\nThe first liquid staking protocol solution was ",(0,i.kt)("a",{parentName:"p",href:"https://blog.lido.fi/staking-ethereum-with-lido/"},"built for Ethereum")," \u2014 and now Lido is expanding to different blockchain networks."),(0,i.kt)("p",null,"The stake deposited to the Lido contract on Solana is distributed to these validators following a logic similar to the Lido ",(0,i.kt)("a",{parentName:"p",href:"https://lido.fi/static/Lido:Ethereum-Liquid-Staking.pdf"},"(stETH) on Ethereum"),". Lido on Solana has a fee mechanism similar to that on Ethereum which allows splitting fees between node operators and the Lido treasury (e.g. to be used for the insurance fund).\nLido\u2019s decentralized organization brings together the industry\u2019s top staking providers, decentralized finance projects, and investors. The Lido DAO eliminates dependence on a centralized authority, thereby removing the risk of a single point of failure. Distributed governance also fosters a stronger community!"),(0,i.kt)("h2",{id:"governance-rewards"},"Governance Rewards"),(0,i.kt)("p",null,"A portion of the rewards goes to the Lido DAO treasury. The amount that goes to the Lido DAO treasury can be potentially used for different purposes"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},"Revenue share to maintainers (20% of the portion going to the treasury, see also the full Chorus One proposal to the Lido DAO)"),(0,i.kt)("li",{parentName:"ul"},"Insurance"),(0,i.kt)("li",{parentName:"ul"},"Grants"),(0,i.kt)("li",{parentName:"ul"},"Value Accrual to LDO")),(0,i.kt)("p",null,"The Lido DAO is the deciding authority on the various parameters of the ecosystem. Things like fees, upgrade approvals, validator set, voting mechanisms, etc. are decided by the DAO. It is in the DAO\u2019s charter to make the system run smoothly and it does so through the process of voting. To be a voter one must possess the governance token, LDO. The amount of LDO determines the weight of your vote."),(0,i.kt)("p",null,"Lido DAO\u2019s governance is a key aspect of the ecosystem and holds the key to the success of Lido for Solana."))}u.isMDXComponent=!0}}]);