---
title: Location Tree Test
description: An ordered look at MD parsing
---

//# Overview
Lorpem ipsum dolor sot

## Landlocked
Lorem ipsum odor amet, consectetuer adipiscing elit. Nisl efficitur inceptos sagittis class conubia felis. Quis luctus ac donec dignissim proin; viverra lobortis mauris habitasse. Lectus aliquet porta vel sit elementum. Posuere dignissim litora ornare eros hac eget. Suscipit elementum pretium vehicula morbi diam tristique suspendisse lobortis. Inceptos consectetur facilisi interdum dui sapien scelerisque lectus lorem rhoncus.

### Switzerland
Nisl mauris vel nascetur auctor lobortis lacinia dis penatibus. Aenean ultrices eu sagittis, vel vestibulum vehicula. Leo pellentesque aliquam placerat nisl rutrum metus. Natoque vivamus vestibulum himenaeos commodo eu dui scelerisque tortor feugiat. At ac quis, leo potenti ultrices curabitur taciti. Montes blandit arcu taciti integer; finibus vel dui fames. Turpis velit bibendum nostra eu magnis phasellus elementum dolor. Velit nisl volutpat malesuada magnis convallis eros nulla diam.

#### Geneva
Vivamus vulputate posuere ultricies eu nibh ad bibendum. Lobortis nostra sem ridiculus magna viverra blandit suspendisse? Et hac blandit curabitur netus, viverra purus eros nisl. Nisl fringilla vestibulum leo faucibus malesuada gravida. Molestie tristique lectus massa porttitor libero duis pellentesque aliquam. Ipsum posuere duis phasellus sodales eget eleifend. Aporttitor erat convallis placerat lacinia fames curae.

##### Old Town
Nunc metus et feugiat vehicula magnis curae, litora platea. Ullamcorper condimentum bibendum tortor montes facilisis taciti aliquam. Netus magnis velit pharetra congue venenatis orci aenean. Ullamcorper lobortis maecenas ut molestie molestie aliquet interdum facilisi. Efficitur tellus viverra morbi tincidunt orci suscipit imperdiet torquent. Cubilia nascetur vitae purus bibendum pulvinar. Pharetra inceptos dolor donec nullam adipiscing vitae eu.

###### Cathédrale Saint-Pierre
Lorem ipsum odor amet, consectetuer adipiscing elit. Ultrices nisi lobortis potenti imperdiet; neque efficitur? Proin platea purus suspendisse leo vitae a habitasse ridiculus. Suscipit iaculis cursus litora lobortis nam parturient metus. Orci suspendisse quisque platea curae ullamcorper. Volutpat senectus malesuada ac hac facilisis felis dui. Tristique sed aliquam cras aliquam laoreet quam aenean. Amet auctor imperdiet sem lacinia ac ultricies. Aptent quam condimentum venenatis in facilisis aliquet. Fusce maecenas venenatis morbi; pretium auctor efficitur taciti ac.

### Bolivia
Id nisi purus molestie sed; ex torquent. Conubia nisl inceptos maximus; auctor molestie morbi class aliquet. Fames purus commodo habitant ligula sapien neque nascetur inceptos. Nibh ultricies tortor porttitor felis lacus maecenas vestibulum molestie justo. Risus enim enim ut diam ultricies sollicitudin imperdiet. Vel posuere dapibus nunc; cursus urna ac. Egestas luctus conubia ipsum sollicitudin aptent platea mi. Finibus luctus porta mi cursus, nulla curabitur aptent nostra.

###### Puerta del Sol
Pulvinar orci eu et finibus lectus lectus himenaeos. Etiam luctus luctus mi justo, imperdiet dictum. Pulvinar platea nec himenaeos, habitasse donec mi. Nostra potenti habitasse magnis litora; nulla curabitur facilisi mus. Ridiculus vestibulum augue taciti a gravida phasellus eleifend sollicitudin dis. Ad dis mi enim sem sit gravida. Lobortis sem non nullam metus curabitur, elementum lectus. Duis massa a ac nec primis fames quis maximus.

## Islands
Mus sociosqu ultricies vehicula nibh cubilia. Fusce montes parturient efficitur ullamcorper curabitur est vestibulum. Sociosqu massa odio sollicitudin cubilia facilisi platea curae. Primis proin malesuada parturient eu, pulvinar varius curae. Enim hendrerit tempor sapien congue nostra porta finibus. Taciti bibendum pulvinar vitae nulla eu. Nostra class dui penatibus vestibulum nec proin a. Curae per ullamcorper mattis maximus; penatibus massa litora. Sit eget pretium odio laoreet mauris himenaeos blandit ac.

### Marine
Felis nec non viverra commodo phasellus. Augue enim massa enim ornare orci pharetra facilisi mollis nulla. Laoreet porta morbi porta integer est, montes aptent hac. Lacinia commodo malesuada arcu nostra integer est consequat torquent. Morbi in eros condimentum euismod, taciti volutpat nisi. Imperdiet turpis elit maximus sapien; ullamcorper ultricies ridiculus. Arcu mi iaculis curabitur penatibus sed ullamcorper non euismod.

#### Australia
Lorem ipsum odor amet, consectetuer adipiscing elit. Per rhoncus erat lacus vel, ultricies ex. Ad pretium ligula maximus habitasse tincidunt adipiscing proin. Sed cubilia pellentesque iaculis conubia quis. Natoque eleifend vestibulum nibh placerat lectus et. Et turpis egestas quis torquent, taciti inceptos. Lacinia magnis mauris auctor malesuada tellus.

### Fresh Water
Vulputate ad nec eget arcu; parturient curae ornare lacinia. Rhoncus senectus etiam, nunc integer inceptos nibh. Porta tristique integer integer vitae fames non parturient. Aturpis tristique integer vel orci a accumsan cras. Aliquet congue conubia consequat malesuada proin lacinia euismod vivamus eu. Ante iaculis ornare leo tempus efficitur condimentum lacinia felis. Porta habitasse nostra praesent interdum orci semper etiam inceptos. Mauris consequat orci, commodo orci molestie vulputate congue. Bibendum eros suscipit semper praesent litora nunc tincidunt nisi.

  Visual references for algorithm construction
```
    [
      lvl: 2, title: "Landlocked"
      lvl: 3, title: "Switzerland"
      lvl: 4, title: "Geneva"
      lvl: 5, title: "Old Town"
      lvl: 6, title: "Cathédrale Saint-Pierre"
      lvl: 3, title: "Bolivia"
      lvl: 2, title: "Islands"
      lvl: 3, title: "Marine"
      lvl: 4, title: "Australia"
      lvl: 3, title: "Fresh Water"
    ]
```

```
    [] Lorem Ipsum Test 
    │    An ordered look at MD parsing
    │
    ├── Landlocked
    │   ├── Switzerland
    │   │   └── Geneva
    │   │       └── Old Town
    │   │           └── Cathédrale Saint-Pierre
    │   └── Bolivia
    └── Island
        ├── Marine
        │   └── Australiae
        └── Fresh Water
```

```
       Lorem Ipsum Test
              |
         ------------
         |          |
        H2         H2
         |          |
     -------      -------
     |     |      |     |
    H3    H3      H3    H3
           |
           H4
```  
