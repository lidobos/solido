(self.webpackChunkdocs=self.webpackChunkdocs||[]).push([[372],{3905:function(e,t,o){"use strict";o.d(t,{Zo:function(){return c},kt:function(){return f}});var r=o(7294);function n(e,t,o){return t in e?Object.defineProperty(e,t,{value:o,enumerable:!0,configurable:!0,writable:!0}):e[t]=o,e}function i(e,t){var o=Object.keys(e);if(Object.getOwnPropertySymbols){var r=Object.getOwnPropertySymbols(e);t&&(r=r.filter((function(t){return Object.getOwnPropertyDescriptor(e,t).enumerable}))),o.push.apply(o,r)}return o}function a(e){for(var t=1;t<arguments.length;t++){var o=null!=arguments[t]?arguments[t]:{};t%2?i(Object(o),!0).forEach((function(t){n(e,t,o[t])})):Object.getOwnPropertyDescriptors?Object.defineProperties(e,Object.getOwnPropertyDescriptors(o)):i(Object(o)).forEach((function(t){Object.defineProperty(e,t,Object.getOwnPropertyDescriptor(o,t))}))}return e}function s(e,t){if(null==e)return{};var o,r,n=function(e,t){if(null==e)return{};var o,r,n={},i=Object.keys(e);for(r=0;r<i.length;r++)o=i[r],t.indexOf(o)>=0||(n[o]=e[o]);return n}(e,t);if(Object.getOwnPropertySymbols){var i=Object.getOwnPropertySymbols(e);for(r=0;r<i.length;r++)o=i[r],t.indexOf(o)>=0||Object.prototype.propertyIsEnumerable.call(e,o)&&(n[o]=e[o])}return n}var l=r.createContext({}),d=function(e){var t=r.useContext(l),o=t;return e&&(o="function"==typeof e?e(t):a(a({},t),e)),o},c=function(e){var t=d(e.components);return r.createElement(l.Provider,{value:t},e.children)},p={inlineCode:"code",wrapper:function(e){var t=e.children;return r.createElement(r.Fragment,{},t)}},u=r.forwardRef((function(e,t){var o=e.components,n=e.mdxType,i=e.originalType,l=e.parentName,c=s(e,["components","mdxType","originalType","parentName"]),u=d(o),f=n,h=u["".concat(l,".").concat(f)]||u[f]||p[f]||i;return o?r.createElement(h,a(a({ref:t},c),{},{components:o})):r.createElement(h,a({ref:t},c))}));function f(e,t){var o=arguments,n=t&&t.mdxType;if("string"==typeof e||n){var i=o.length,a=new Array(i);a[0]=u;var s={};for(var l in t)hasOwnProperty.call(t,l)&&(s[l]=t[l]);s.originalType=e,s.mdxType="string"==typeof e?e:n,a[1]=s;for(var d=2;d<i;d++)a[d]=o[d];return r.createElement.apply(null,a)}return r.createElement.apply(null,o)}u.displayName="MDXCreateElement"},3099:function(e,t,o){"use strict";o.r(t),o.d(t,{frontMatter:function(){return a},metadata:function(){return s},toc:function(){return l},default:function(){return c}});var r=o(2122),n=o(9756),i=(o(7294),o(3905)),a={id:"overview",title:"overview",description:"Overview of LIDO for Solana",keywords:["lido","solido","solana"],slug:"/",sidebar_position:1},s={unversionedId:"overview",id:"overview",isDocsHomePage:!1,title:"Overview",description:"Overview of LIDO for Solana",source:"@site/docs/overview.md",sourceDirName:".",slug:"/",permalink:"/docs/",version:"current",sidebarPosition:1,frontMatter:{id:"overview",title:"overview",description:"Overview of LIDO for Solana",keywords:["lido","solido","solana"],slug:"/",sidebar_position:1},sidebar:"solidoSidebar",next:{title:"Staking Overview",permalink:"/docs/Guides/Staking/Staking"}},l=[{value:"Lido for Solana",id:"lido-for-solana",children:[]},{value:"How Lido for Solana works",id:"how-lido-for-solana-works",children:[]}],d={toc:l};function c(e){var t=e.components,a=(0,n.Z)(e,["components"]);return(0,i.kt)("wrapper",(0,r.Z)({},d,a,{components:t,mdxType:"MDXLayout"}),(0,i.kt)("h2",{id:"lido-for-solana"},"Lido for Solana"),(0,i.kt)("p",null,"'Lido for Solana' is a Lido-DAO governed liquid staking protocol for the Solana blockchain. Anyone who stakes their SOL tokens with Lido will be issued an on-chain representation of SOL staking position with Lido validators, called ",(0,i.kt)("strong",{parentName:"p"},"stSOL"),". We will work to integrate stSOL widely into the Solana DeFi ecosystem to enable stSOL users to make use of their staked assets in a variety of applications."),(0,i.kt)("p",null,"Lido for Solana gives you:"),(0,i.kt)("ul",null,(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("strong",{parentName:"li"},"Liquidity")," \u2014 No delegation/activation delays and the ability to sell your staked tokens"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("strong",{parentName:"li"},"One-click staking")," \u2014 No complicated steps"),(0,i.kt)("li",{parentName:"ul"},(0,i.kt)("strong",{parentName:"li"},"Decentralized security")," \u2014 Assets spread across the industry\u2019s leading validators chosen by the Lido DAO")),(0,i.kt)("h2",{id:"how-lido-for-solana-works"},"How Lido for Solana works"),(0,i.kt)("p",null,"Lido for Solana not only makes it very easy to stake but also provides further utility through stSOL. Let\u2019s look at the process in slight detail. A SOL token holder connects their wallet to an interface that supports Lido (one will e.g. be hosted at ",(0,i.kt)("a",{parentName:"p",href:"https://stake.lido.fi"},"https://stake.lido.fi"),") and deposits their tokens into the Lido program. They immediately receive stSOL tokens that represent a share of the total pool. Every user\u2019s tokens are first held in a pool controlled by the Lido program"),(0,i.kt)("p",null,(0,i.kt)("img",{alt:"How Solido works",src:o(6294).Z})),(0,i.kt)("p",null,"The Lido program collects the deposited SOL and releases the newly minted stSOL to the user. Beneath the layer, the Lido Program leverages the Stake Pool Program Library to distribute this SOL uniformly across validators participating in the Lido Program. When these delegations accrue rewards on the allotted stake, the total SOL under stake pool management increases and this increases the value of stSOL tokens. The Lido DAO governs the Lido Program and the underlying Stake Pool program \u2014 and also controls the list of validators that are part of this program."))}c.isMDXComponent=!0},6294:function(e,t,o){"use strict";t.Z=o.p+"assets/images/howsolidoworks-cfe263fe5d8d5319d48339b32e5e47e3.png"}}]);