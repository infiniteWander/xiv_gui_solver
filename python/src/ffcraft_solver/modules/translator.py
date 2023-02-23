from ffcraft_solver.modules import log

loggers = log.Loggers

supported_languages = {
    "en": {
        'basicSynth': 'Basic Synthesis',
        'basicSynth2': 'Basic Synthesis',
        'basicTouch': 'Basic Touch',
        'mastersMend': "Master's mend",
        'hastyTouch': 'Hasty Touch',
        'rapidSynthesis': 'Rapid Synthesis',
        'wasteNot': 'Waste Not',
        'veneration': 'Veneration',
        'standardTouch': 'Standard Touch',
        'greatStrides': 'Great Strides',
        'innovation': 'Innovation',
        'finalAppraisal': 'Final Appraisal',
        'wasteNot2': 'Waste Not II',
        'byregotsBlessing': "Byregot's Blessing",
        'preciseTouch': 'Precise Touch',
        'muscleMemory': 'Muscle Memory',
        'carefulSynthesis': 'Careful Synthesis',
        'manipulation': 'Manipulation',
        'prudentTouch': 'Prudent Touch',
        'focusedSynthesis': 'Focused Synthesis',
        'focusedTouch': 'Focused Touch',
        'reflect': 'Reflect',
        'preparatoryTouch': 'Preparatory Touch',
        'groundwork': 'Groundwork',
        'delicateSynthesis': 'Delicate Synthesis',
        'intensiveSynthesis': 'Intensive Synthesis',
        'trainedEye': 'Trained Eye',
        'advancedTouch': 'Advanced Touch',
        'prudentSynthesis': 'Prudent Synthesis',
        'trainedFinesse': 'Trained Finesse'
    }
}


def translate_list(actions: list, lang: str):
    output = []
    print(actions)
    if lang in supported_languages:
        dictionary = supported_languages[lang]
        for action in actions:
            try:
                output.append(dictionary[action])
            except KeyError:
                output.append(supported_languages["en"][action])
                loggers.log('Translation not found. Defaulting to English.\n'
                            '    Macros will probably be broken because of this.')


    else:
        print(f'Log: Language code "{lang}" not recognised.')

    return output

