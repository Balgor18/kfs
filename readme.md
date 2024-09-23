# Kernel From Scratch 1

The major docs used for this project is [OsDev][Wiki OsDev].


## Qu'est ce qu'un OS ?
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

https://elearning.intra.42.fr/notions/kfs-1/videos/kfs-1
https://wiki.osdev.org

CLI kvm pour lancer le kernel dans le term
`sudo kvm -m 2048 -s -hda ./test.img -redir tcp:2323::23 -curses`

Debugging :

Ouvrir KVM puis gdb dans un autre terminal
`>>> target remote:1234`

https://sourceware.org/gdb/onlinedocs/gdb.html

[Wiki OsDev]: https://wiki.osdev.org/Introduction "OsDev Wiki"
https://os.phil-opp.com/
https://os.phil-opp.com/edition-1/
https://www.youtube.com/playlist?list=PL980gcR1LE3LBuWuSv2CL28HsfnpC4Qf7
https://www.youtube.com/watch?v=rH5jnbJ3tL4
https://intermezzos.github.io/book/first-edition/what.html
https://samypesse.gitbook.io/how-to-create-an-operating-system