# Kernel From Scratch (KFS)

## Prerequis for the project :
- [QEMU][QEMU Install]
- [Rust][Rust Install]

## First part of KFS
The major docs used for this project is [OsDev][Wiki OsDev].
I start the project with this [Blog][Rust Blog] but this doc got some few error and i don't want to copy paste all the code. After a long search i found all the points need by the subject :
- Print on screen using [VGA][Printing on screen]. How it's [work][Explanation VGA text mode].
- [Layout][Layout table] of the keyboard.
- [Keyboard input][felixcloutier I/O Explain] on screen. More [docs][PS2 controller]
- Implement [Cursor][Cursor I/O]


## Second part of KFS
In this part of the project we need to create the GDT(Global Descriptor Table).  
We used this [docs][GDT OSDEV] and a [video][GDT youtube].  
<!-- ## Qu'est ce qu'un OS ?
Un logiciel controllant le systéme informatique et les ressources.
Fonction principale d'un OS :
- Gestion de la mémoire et des autres ressources système.
- Imposer des politiques de sécurité et d'accès.
- Planification et multiplexage des processus et des threads.
- Lancement et fermeture dynamiques des programmes utilisateurs.
- Fournir une interface utilisateur de base et une interface de programmeur d'application.

Pour rappel un OS n'est pas :
- Le Hardware
- Une application (ex: Note, bloc note, Brave, ...)
- Suite d'utilitaire (ex : GNU)
- Une environnement de Dev (ex : IDE)
- Interface graphique 

Ils peuvent être distribués dans l'OS mais ne font pas partie de l'OS.

## Qu'est ce qu'un noyau (Kernel) :

Le kernel est une part de l'OS que nous ne verrons jamais. Il permet à tout les programmes de s'éxecuter.
Il gére les events du hardware comme du software et manage l'accés des ressources.

Le noyau va faire de l'abstraction pour les fichiers, les processus, les sockets, les repertoires et bien d'autre encore.

-->
<!--

## Qu'est ce qu'un GDT (Global Descriptor Table) ? 
Structure de données utilisée par le processeur dans les OS qui sont en mode protegé.
Elle va mettre en place des descriptuer de segment.
Un descripteur de segment est un bloc qui decrit ce que fais le segements de mémoire utilisés par le système.

La GDT permet de faire :
- Définir les droits d'accés
- Faire de la transistion de Mode. (Une partie Kernel. Une autre User).

Schéma :
+-------------------------+
|        GDT               |
+-------------------------+
| Descripteur 0 (Null)    | <-- Toujours à 0, non utilisé
+-------------------------+
| Descripteur 1 (Code)    | <-- Segment de code
+-------------------------+
| Descripteur 2 (Données) | <-- Segment de données
+-------------------------+
| Descripteur 3 (Stack)   | <-- Segment de pile
+-------------------------+
| ...                     | <-- Autres descripteurs si nécessaire
+-------------------------+

Descripteur 0 (Null) : Le premier descripteur est toujours nul et n'est pas utilisé. Il sert de point de référence.
Descripteur 1 (Code) : Définit le segment contenant le code exécutable du système ou des applications.
Descripteur 2 (Données) : Définit le segment contenant les données utilisées par le système ou les applications.
Descripteur 3 (Stack) : Définit le segment utilisé pour la pile (stack), essentielle pour les appels de fonctions et la gestion des variables locales.
Autres Descripteurs : La GDT peut contenir d'autres descripteurs pour des segments supplémentaires, tels que des segments pour les tâches, les périphériques, etc.

Comment utilisée la GDT dans le Kernel ?
- Initialisation : Au démarrage, le Kernel initialise la GDT avec les descripteurs nécessaires.
- Chargement des Registres : Le Kernel charge le registre GDTR (Global Descriptor Table Register) avec l'adresse et la taille de la GDT.
- Sélection des Segments : Lorsqu'un processus est exécuté, le Kernel utilise les descripteurs de la GDT pour sélectionner les segments appropriés pour le code, les données et la pile.
- Protection et Isolation : Grâce aux droits d'accès définis dans les descripteurs, le Kernel assure que les processus ne peuvent pas accéder ou modifier la mémoire qui ne leur est pas allouée.

Conclu :
La GDT est essentielle pour la gestion de la mémoire en mode protégé, offrant une structure organisée pour définir et contrôler l'accès aux différents segments de mémoire. Cela permet au Kernel de maintenir la stabilité et la sécurité du système en isolant les processus et en protégeant les ressources critiques.

OFFSET = x << dans un decalage binaire.

Example address
0x00cf9a000000ffff
Attention address en Hexa donc 1 chiffre Hexa = 4 bit

| 63 56 | 55 52 | 51 48 | 47 40 | 39 32 | 31 16 | 15 0 |
| ----- | ----- | ----- | ----- | ----- | ----- | ---- |
| Base | Flags | Limit |  Access Byte | Base| Base | Limit |
|  00  |   c   |   f   |      9a      |  00 | 0000 |  ffff |

Pour mieux comprendre chacun des blocs. Je vous renvoie vers la [Doc][GDT OSDev].

-->

## Third part of KFS
In this part of the project we need to activate Paging.  
We used this [docs][Paging OSDEV] and a [video][Paging Video].  
[Intel Doc][Intel Docs books] for help see page 4-12 4-13.  
Another help for this part of the project is this [rust blog][[Rust blog Paging]].  

<!-- KFS1 -->
[QEMU Install]: https://www.qemu.org/download/
[Rust Install]: https://www.rust-lang.org/tools/install
[Wiki OsDev]: https://wiki.osdev.org/Introduction "OsDev Wiki"
[Rust Blog]: https://os.phil-opp.com/ "Bluid kernel in Rust"
[Printing on screen]: https://os.phil-opp.com/printing-to-screen/ "VGA Part"
[Explanation VGA text mode]: https://en.wikipedia.org/wiki/VGA_text_mode "VGA Wikipedia"
[felixcloutier I/O Explain]: https://www.felixcloutier.com/x86/in "I/O Explanation"
[Cursor I/O]: https://wiki.osdev.org/Text_Mode_Cursor#Without_the_BIOS "Cursor explain"
[PS2 controller]: https://wiki.osdev.org/%228042%22_PS/2_Controller#PS/2_Controller_IO_Ports "PS2 controller"
[Layout table]: https://users.utcluj.ro/~baruch/sie/labor/PS2/Scan_Codes_Set_1.htm "Layout Keyboard Qwerty"

<!-- KFS2 -->
[GDT OSDEV]: https://wiki.osdev.org/Global_Descriptor_Table "GDT"
[GDT youtube]: https://www.youtube.com/watch?v=0nT_2aIOTq8&t=896s "Explain Youtube GDT"

<!-- KFS3 -->
[Paging OSDEV]: https://wiki.osdev.org/Paging  
[Intel Docs books]: https://cdrdv2.intel.com/v1/dl/getContent/671447  
<!-- Page 4-12 4-13 -->
[Paging Video]: https://www.youtube.com/watch?v=B1wJJNITvkY  
[Rust blog Paging]: https://os.phil-opp.com/paging-introduction/  

<!-- KFS4 -->

<!-- Few links useful -->
[GDB website]: https://sourceware.org/gdb/onlinedocs/gdb.html

[Wildcard rust]: https://doc.rust-lang.org/reference/patterns.html#wildcard-pattern
[Ignore value]: https://doc.rust-lang.org/book/ch18-03-pattern-syntax.html#ignoring-an-entire-value-with-_
[Video GDT]: https://www.youtube.com/watch?v=Wh5nPn2U_1w
[Operateur ASM]: https://www.felixcloutier.com/x86/mov#operation
[Multiboot Header]: https://www.gnu.org/software/grub/manual/multiboot/multiboot.html#Specification  

<!-- https://github.com/rust-embedded-community/pc-keyboard/blob/HEAD/src/layouts/azerty.rs -->

<!-- Launch system
cargo build && qemu-system-i386 -kernel target/kfs/debug/kfs
 -->
<!-- REMIND Debug data part 

Debug system
Launch with : qemu-system-i386 -kernel target/kfs/debug/kfs -s -S


Launch :
gdb target/kfs/release/kfs
target remote :1234
b // Breakpoint on your function
continue // Start the debugging
-->

<!-- 
directory.rs:11
mod.rs:15
mod.rs:19
mod.rs:55
 -->