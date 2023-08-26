# Welcome Future Contributor!
Our vision is to create a social-networking platform that mirrors the functionality of Facebook while also considering the needs of the community.  We hope to be one of many federated applications working towards a better, friendlier, social Internet.

We welcome help from technical, as well as non-technical folks equally.  Even if you do not code, there is a growing list of items that you may still be able to help out with. <br />

Please do not hesitate to ask questions, or volunteer for things that you might be interested in poking at.
Also checkout the [Developer Documentation](/doc/development) for more information on specific development targets.

--------
## Goals
- Immediate: Update code-base to a working state
- Immediate: Finalize UI-framework decision (Yew vs. Slint vs. Leptos)

## Requirements for Minimum Viable Product
* Basic user profile
	* User can provide a:
		* Name
		* Avatar (jpg or png)
        * A short, optional, Bio
* Useable local timeline
	* User can create a new post, containing
		* Text up to 1000 characters
		* One image (jpg or png)
	* User can reply to an existing post
	* User can Favorite/Unfavorite a post
	* User can boost a post
		* Boost/Repost
    * User can Subject Line a topic (SL)
        * SL's can fold/unfold    

## Requirements for Alpha-build
- Veilid networking implementation

## Stretch Goals for Alpha
- UI Accessibility - [Web Accessibility Evaluation Tool (WAVE)](http://wave.webaim.org/))
- Color scheme (colorblind- and low-vision-friendly). [Contrast Checker](https://webaim.org/resources/contrastchecker/)
- Clarify and beautify Github documentation. [Github's Markdown Guide](https://guides.github.com/features/mastering-markdown/)

# Future Goals
- Bridge connection to [Plume](https://github.com/Plume-org/Plume)
- Bridge connection to Matrix (or similar) for chat service
- Bridge connection to [PeerTube](https://github.com/Chocobozzz/PeerTube) for video sharing

## Other items under consideration
- End-to-End Encryption
- IP FileSystem for file sharing (?)