# README #

*MESCC Dissertation  LaTeX Template - Version 0.1 (April/2023)*

This template provides the main formatting rules for the Master dissertation document of the [Critical Computing Systems Engineering Master](https://www.isep.ipp.pt/mescc/), Computer Engineering Department (DEI), School of Engineering (ISEP) of the Polytechnic of Porto (IPP).

**You can fork this repository to make your own dissertation based on this template.**

This template is an adaptation of the TMDEI/ISEP style (Version 0.2, Feb/2022), and modified to align with the ISEP dissertation guidelines (Regulamento n.o 1030/2023, 21 September 2023) by Luis Miguel Pinho and António Barros. 

The TMDEI/ISEP template was produced by Nuno Pereira and Paulo Baltarejo (DEI/ISEP), and can be downloaded from [the repository](https://github.com/MEI-ISEP/tmdei-dissertation-template).

Originally based on MastersDoctoralThesis version 1.2 by Vel (vel@latextemplates.com) and Johannes Böttcher, downloaded from [LaTeXTemplates](http://www.LaTeXTemplates.com) in November/2015. Adapted to TMDEI/ISEP style (Dec/2015) by Nuno Pereira and Paulo Baltarejo (DEI/ISEP).

## Also on Overleaf ##

"Overleaf is an online LaTeX and Rich Text collaborative writing and publishing tool that makes the whole process of writing, editing and publishing scientific documents much quicker and easier".

This template is also available as a [template at Overleaf](https://www.overleaf.com/read/rwxfxtzjznck).

## How do I get set up? ##

Just fork the repository and use it. You will need LaTeX tools installed in your system, with the packages needed by the template (more details bellow).

### LaTeX Distribution

LaTeX is available for many systems including Windows, Linux and Mac OS X. Check the webpage for the LaTex project for more information: <https://latex-project.org/ftp.html>.

Make sure you have the following tools installed: **pdflatex**, **makeglossaries**, **biber**, **latexmk**.



### LaTeX Packages Needed

| Package | Obs |
|---------|-----|
|etoolbox| |
|xparse| |
|parskip| |
|babel|Required for automatically changing names of document elements to languages besides english|
|scrbase|Required for handling language-dependent names of sections/document elements|
|scrhack|Loads fixes for various packages|
|setspace|Required for changing line spacing|
|longtable|Required for tables that span multiple pages (used in the symbols, abbreviations and physical constants pages)|
|siunitx|Required for \SI commands|
|graphicx|Required to include images|
|xcolor|Required for extra color names|
|booktabs|Required for better table rules|
|lmodern|Latin Modern fonts|
|opensans|Default (sans serif) font|
|inconsolata|Mono space font for listings|
|inputenc|Required for inputting portuguese characters|
|fontenc|Output font encoding for portuguese characters|
|csquotes|Required to generate language-dependent quotes in the bibliography|
|algorithm|Required for algorithms|
|algpseudocode|Part of algorithmicx package, required to customize the layout of algorithms|
|listings|Required for code listings|
|glossaries|Required to define acronyms and make glossaries|
|caption|Required for customising the captions|
|geometry| |
|scrlayer-scrpage| |
|biblatex|Required for citations and bibliography|
|hyperref| |
|tikz|Required for creating graphics programmatically (can be removed if not used)|
|pgfplots|Required for drawing high--quality function plots (can be removed if not used)|

## Who do I talk to? ##

* Luis Miguel Pinho (LMP@isep.ipp.pt)
* António Barros (AMB@isep.ipp.pt)
