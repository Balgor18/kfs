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


### Debug data
Il faut lancer le projet avec Kvm. Pour cela il faut une machine virtuel Ubuntu.

Ensuite il faut un disque dur comme un fichier `.img`.

https://elearning.intra.42.fr/notions/kfs-1/videos/kfs-1 -->

https://sourceware.org/gdb/onlinedocs/gdb.html

[QEMU Install]: https://www.qemu.org/download/
[Rust Install]: https://www.rust-lang.org/tools/install
[Wiki OsDev]: https://wiki.osdev.org/Introduction "OsDev Wiki"
[Rust Blog]: https://os.phil-opp.com/edition-1/ "Bluid kernel in Rust"
[Printing on screen]: https://os.phil-opp.com/printing-to-screen/ "VGA Part"
[Explanation VGA text mode]: https://en.wikipedia.org/wiki/VGA_text_mode "VGA Wikipedia"
[felixcloutier I/O Explain]: https://www.felixcloutier.com/x86/in "I/O Explanation"
[Cursor I/O]: https://wiki.osdev.org/Text_Mode_Cursor#Without_the_BIOS "Cursor explain"
[PS2 controller]: https://wiki.osdev.org/%228042%22_PS/2_Controller#PS/2_Controller_IO_Ports "PS2 controller"
[Layout table]: https://users.utcluj.ro/~baruch/sie/labor/PS2/Scan_Codes_Set_1.htm "Layout Keyboard Qwerty"

